use serde::Serialize;
use serde_json::{Value, json};
use serde_wasm_bindgen::Serializer;
use std::collections::HashMap;
use wasm_bindgen::prelude::{JsValue, wasm_bindgen};

use finite_element_method::FEM;

mod methods_for_nodes_data_handle;
use methods_for_nodes_data_handle::extract_nodes_from_input_data;

mod methods_for_truss_elements_data_handle;
use methods_for_truss_elements_data_handle::extract_truss_elements_from_input_data;

mod methods_for_beam_elements_data_handle;
use methods_for_beam_elements_data_handle::extract_beam_elements_from_input_data;

mod methods_for_plate_elements_data_handle;
use methods_for_plate_elements_data_handle::extract_plate_elements_from_input_data;

mod methods_for_bc_data_handle;
use methods_for_bc_data_handle::{
    dof_parameter_into_str, element_force_component_into_str, extract_nodal_bc_from_input_data,
    extract_udls_on_beams_from_input_data, extract_udls_on_plates_from_input_data,
};

mod types;
use types::FEFloat;

mod enums;
use enums::Preconditioner;

mod webgpu;

use crate::webgpu::{
    WebGpuCtx, find_ua_vector_iterative_pcg_block_jacobi_sparse_webgpu, init_webgpu,
};

#[wasm_bindgen]
pub struct Solver {
    job_name: String,
    truss_elements_group_number: u32,
    beam_elements_group_number: u32,
    plate_elements_group_number: u32,
    node_number_node_name_map: HashMap<u32, String>,
    fem: FEM<FEFloat>,
    ctx: WebGpuCtx,
}

#[wasm_bindgen]
impl Solver {
    pub async fn create(
        job_name: String,
        rel_tol: FEFloat,
        abs_tol: FEFloat,
        truss_elements_group_number: u32,
        beam_elements_group_number: u32,
        plate_elements_group_number: u32,
        mesh: JsValue,
    ) -> Result<Solver, JsValue> {
        let serialized_data: Value = serde_wasm_bindgen::from_value(mesh).or(Err(
            JsValue::from("Solver: Mesh data could not be serialized!"),
        ))?;

        let extracted_statistics_data = serialized_data.get("statistics").ok_or(JsValue::from(
            "Solver: Statistics data could not be extracted!",
        ))?;
        let statistics_data = extracted_statistics_data.as_object().ok_or(JsValue::from(
            "Solver: Statistics data could not be converted into object!",
        ))?;
        let nodes_number = statistics_data
            .get("nodes_number")
            .ok_or(JsValue::from(
                "Solver: Nodes number could not be extracted from statistics data!",
            ))?
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Solver: Nodes number could not be converted to u32!",
            )))?;
        let mut fem = FEM::create(rel_tol, abs_tol, nodes_number);

        let mut node_number_node_name_map = HashMap::new();
        let mut node_name_node_number_map = HashMap::new();

        let extracted_nodes_data = serialized_data
            .get("nodes")
            .ok_or(JsValue::from("Solver: Nodes data could not be extracted!"))?;
        extract_nodes_from_input_data(
            extracted_nodes_data,
            &mut fem,
            &mut node_number_node_name_map,
            &mut node_name_node_number_map,
        )?;

        let extracted_truss_elements_data = serialized_data.get("truss_elements").ok_or(
            JsValue::from("Solver: Truss elements data could not be extracted!"),
        )?;
        extract_truss_elements_from_input_data(
            extracted_truss_elements_data,
            &mut fem,
            &mut node_name_node_number_map,
        )?;

        let extracted_beam_elements_data = serialized_data.get("beam_elements").ok_or(
            JsValue::from("Solver: Beam elements data could not be extracted!"),
        )?;
        extract_beam_elements_from_input_data(
            extracted_beam_elements_data,
            &mut fem,
            &mut node_name_node_number_map,
        )?;

        let extracted_plate_elements_data = serialized_data.get("plate_elements").ok_or(
            JsValue::from("Solver: Plate elements data could not be extracted!"),
        )?;
        extract_plate_elements_from_input_data(
            extracted_plate_elements_data,
            &mut fem,
            &mut node_name_node_number_map,
        )?;

        let extracted_nodal_bc_data =
            serialized_data
                .get("boundary_conditions")
                .ok_or(JsValue::from(
                    "Solver: Nodal boundary conditions data could not be extracted!",
                ))?;
        extract_nodal_bc_from_input_data(
            extracted_nodal_bc_data,
            &mut fem,
            &mut node_name_node_number_map,
        )?;

        let extracted_udls_on_beams_data = serialized_data
            .get("uniformly_distributed_loads_on_beams")
            .ok_or(JsValue::from(
                "Solver: Uniformly distributed loads on beams data could not be extracted!",
            ))?;
        extract_udls_on_beams_from_input_data(extracted_udls_on_beams_data, &mut fem)?;

        let extracted_udls_on_plates_data = serialized_data
            .get("uniformly_distributed_loads_on_plates")
            .ok_or(JsValue::from(
                "Solver: Uniformly distributed loads on plates data could not be extracted!",
            ))?;
        extract_udls_on_plates_from_input_data(extracted_udls_on_plates_data, &mut fem)?;

        let ctx = init_webgpu()
            .await
            .map_err(|e| JsValue::from_str(&format!("init_webgpu failed: {e}")))?;

        Ok(Solver {
            job_name,
            truss_elements_group_number,
            beam_elements_group_number,
            plate_elements_group_number,
            node_number_node_name_map,
            fem,
            ctx,
        })
    }

    pub fn perform_global_analysis_direct(&mut self, job_name: String) -> Result<(), JsValue> {
        if job_name != self.job_name {
            return Err(JsValue::from(format!("Incorrect job name {job_name}")));
        }

        let separated_stiffness_matrix = self.fem.separate_stiffness_matrix_direct()?;
        let r_a_vector = self
            .fem
            .compose_r_a_vector(separated_stiffness_matrix.get_k_aa_indexes())?;
        let u_b_vector = self
            .fem
            .compose_u_b_vector(separated_stiffness_matrix.get_k_bb_indexes())?;

        let u_a_vector = self.fem.find_ua_vector_direct(
            &separated_stiffness_matrix,
            &r_a_vector,
            &u_b_vector,
        )?;

        let r_r_vector =
            self.fem
                .find_r_r_vector(&separated_stiffness_matrix, &u_a_vector, &u_b_vector)?;

        self.fem.compose_global_analysis_result(
            separated_stiffness_matrix.get_k_aa_indexes(),
            separated_stiffness_matrix.get_k_bb_indexes(),
            &u_a_vector,
            &r_r_vector,
        )?;

        Ok(())
    }

    pub async fn perform_global_analysis_iterative(
        &mut self,
        job_name: String,
        preconditioner_name: String,
        max_iter: usize,
    ) -> Result<usize, JsValue> {
        if job_name != self.job_name {
            return Err(JsValue::from(format!("Incorrect job name {job_name}")));
        }

        let separated_stiffness_matrix_sparse =
            self.fem.separate_stiffness_matrix_sparse_iterative()?;
        let r_a_vector = self
            .fem
            .compose_r_a_vector(separated_stiffness_matrix_sparse.get_k_aa_indexes())?;
        let u_b_vector = self
            .fem
            .compose_u_b_vector(separated_stiffness_matrix_sparse.get_k_bb_indexes())?;

        let preconditioner = Preconditioner::from_str(&preconditioner_name)?;

        let (u_a_vector, iterations) = match preconditioner {
            Preconditioner::Jacobi => self.fem.find_ua_vector_iterative_pcg_jacobi_sparse(
                &separated_stiffness_matrix_sparse,
                &r_a_vector,
                &u_b_vector,
                max_iter,
            )?,
            Preconditioner::BlockJacobi => {
                self.fem.find_ua_vector_iterative_pcg_block_jacobi_sparse(
                    &separated_stiffness_matrix_sparse,
                    &r_a_vector,
                    &u_b_vector,
                    max_iter,
                )?
            }
            Preconditioner::BlockJacobiGpu => {
                find_ua_vector_iterative_pcg_block_jacobi_sparse_webgpu(
                    &separated_stiffness_matrix_sparse,
                    &r_a_vector,
                    &u_b_vector,
                    max_iter,
                    &self.ctx,
                )
                .await?
            }
        };

        let r_r_vector = self.fem.find_r_r_vector_sparse(
            &separated_stiffness_matrix_sparse,
            &u_a_vector,
            &u_b_vector,
        )?;

        self.fem.compose_global_analysis_result(
            separated_stiffness_matrix_sparse.get_k_aa_indexes(),
            separated_stiffness_matrix_sparse.get_k_bb_indexes(),
            &u_a_vector,
            &r_r_vector,
        )?;

        Ok(iterations)
    }

    pub fn extract_global_analysis_result(&mut self, job_name: String) -> Result<JsValue, JsValue> {
        if job_name != self.job_name {
            return Err(JsValue::from(format!("Incorrect job name {job_name}")));
        }

        let extracted_global_analysis_result_data = self
            .fem
            .extract_global_analysis_result()
            .map_err(JsValue::from)?;

        let mut global_analysis_result_data: HashMap<&str, [FEFloat; 12]> = HashMap::new();
        let mut extreme_global_displacements: HashMap<&str, [FEFloat; 2]> = HashMap::new();
        let mut extreme_global_loads: HashMap<&str, [FEFloat; 2]> = HashMap::new();

        for (node_number, dof_parameter, displacement_value, load_value) in
            extracted_global_analysis_result_data.iter()
        {
            let node_name =
                self.node_number_node_name_map
                    .get(node_number)
                    .ok_or(JsValue::from(format!(
                        "Solver: Node number {node_number} does not exist!"
                    )))?;

            if let Some(existed_data_at_node) =
                global_analysis_result_data.get_mut(node_name.as_str())
            {
                existed_data_at_node[*dof_parameter as usize] = *displacement_value;
                existed_data_at_node[*dof_parameter as usize + 6] = *load_value;
            } else {
                let mut data_at_node = [0.0; 12];
                data_at_node[*dof_parameter as usize] = *displacement_value;
                data_at_node[*dof_parameter as usize + 6] = *load_value;
                global_analysis_result_data.insert(node_name, data_at_node);
            }

            if let Some([min_displacement_value, max_displacement_value]) =
                extreme_global_displacements.get_mut(dof_parameter_into_str(dof_parameter))
            {
                if *displacement_value < *min_displacement_value {
                    *min_displacement_value = *displacement_value;
                }

                if *displacement_value > *max_displacement_value {
                    *max_displacement_value = *displacement_value;
                }
            } else {
                extreme_global_displacements.insert(
                    dof_parameter_into_str(dof_parameter),
                    [*displacement_value, *displacement_value],
                );
            }

            if let Some([min_load_value, max_load_value]) =
                extreme_global_loads.get_mut(dof_parameter_into_str(dof_parameter))
            {
                if *load_value < *min_load_value {
                    *min_load_value = *load_value;
                }

                if *load_value > *max_load_value {
                    *max_load_value = *load_value;
                }
            } else {
                extreme_global_loads.insert(
                    dof_parameter_into_str(dof_parameter),
                    [*load_value, *load_value],
                );
            }
        }

        for data_at_node in global_analysis_result_data.values() {
            let u_result =
                (data_at_node[0].powi(2) + data_at_node[1].powi(2) + data_at_node[2].powi(2))
                    .sqrt();
            let r_result =
                (data_at_node[3].powi(2) + data_at_node[4].powi(2) + data_at_node[5].powi(2))
                    .sqrt();
            let f_result =
                (data_at_node[6].powi(2) + data_at_node[7].powi(2) + data_at_node[8].powi(2))
                    .sqrt();
            let m_result =
                (data_at_node[9].powi(2) + data_at_node[10].powi(2) + data_at_node[11].powi(2))
                    .sqrt();
            if let Some([min_displacement_value, max_displacement_value]) =
                extreme_global_displacements.get_mut("u_result")
            {
                if u_result < *min_displacement_value {
                    *min_displacement_value = u_result;
                }

                if u_result > *max_displacement_value {
                    *max_displacement_value = u_result;
                }
            } else {
                extreme_global_displacements.insert("u_result", [u_result, u_result]);
            }

            if let Some([min_displacement_value, max_displacement_value]) =
                extreme_global_displacements.get_mut("r_result")
            {
                if r_result < *min_displacement_value {
                    *min_displacement_value = r_result;
                }

                if r_result > *max_displacement_value {
                    *max_displacement_value = r_result;
                }
            } else {
                extreme_global_displacements.insert("r_result", [r_result, r_result]);
            }

            if let Some([min_load_value, max_load_value]) = extreme_global_loads.get_mut("f_result")
            {
                if f_result < *min_load_value {
                    *min_load_value = f_result;
                }

                if f_result > *max_load_value {
                    *max_load_value = f_result;
                }
            } else {
                extreme_global_loads.insert("f_result", [f_result, f_result]);
            }

            if let Some([min_load_value, max_load_value]) = extreme_global_loads.get_mut("m_result")
            {
                if m_result < *min_load_value {
                    *min_load_value = m_result;
                }

                if m_result > *max_load_value {
                    *max_load_value = m_result;
                }
            } else {
                extreme_global_loads.insert("m_result", [m_result, m_result]);
            }
        }

        let serializer = Serializer::json_compatible();
        let global_analysis_result = json!({
            "global_analysis_result": global_analysis_result_data,
            "extreme_global_displacements": extreme_global_displacements,
            "extreme_global_loads": extreme_global_loads,
        })
        .serialize(&serializer)
        .or(Err(JsValue::from(
            "Solver: Global analysis result could not be composed for extraction!",
        )))?;

        Ok(global_analysis_result)
    }

    pub fn extract_elements_analysis_result(
        &mut self,
        job_name: String,
    ) -> Result<JsValue, JsValue> {
        if job_name != self.job_name {
            return Err(JsValue::from(format!("Incorrect job name {job_name}")));
        }

        let extracted_elements_analysis_result_data = self
            .fem
            .extract_elements_analysis_result()
            .map_err(JsValue::from)?;

        let mut extreme_elements_loads: HashMap<&str, [FEFloat; 2]> = HashMap::new();
        let mut truss_elements_analysis_result_data = HashMap::new();
        let mut beam_elements_analysis_result_data = HashMap::new();
        let mut plate_elements_analysis_result_data = HashMap::new();

        for (element_number, element_analysis_result) in
            extracted_elements_analysis_result_data.iter()
        {
            for (component, load_value) in element_analysis_result.iter() {
                if let Some([min_load_value, max_load_value]) =
                    extreme_elements_loads.get_mut(element_force_component_into_str(component))
                {
                    if *load_value < *min_load_value {
                        *min_load_value = *load_value;
                    }

                    if *load_value > *max_load_value {
                        *max_load_value = *load_value;
                    }
                } else {
                    extreme_elements_loads.insert(
                        element_force_component_into_str(component),
                        [*load_value, *load_value],
                    );
                }
            }

            if element_number
                .to_string()
                .starts_with(&self.truss_elements_group_number.to_string())
            {
                let mut truss_element_analysis_result_data = [0.0 as FEFloat; 10];

                let rotation_matrix_elements = self
                    .fem
                    .get_truss_rotation_matrix_elements(*element_number)
                    .map_err(JsValue::from)?;

                rotation_matrix_elements
                    .iter()
                    .enumerate()
                    .for_each(|(i, v)| truss_element_analysis_result_data[i] = *v);

                if element_analysis_result.len() != 1 {
                    return Err(JsValue::from(&format!(
                        "Solver: Incorrect number of result element loads for truss element {element_number}!",
                    )));
                }
                truss_element_analysis_result_data[9] = element_analysis_result[0].1;

                truss_elements_analysis_result_data
                    .insert(*element_number, truss_element_analysis_result_data);
            } else if element_number
                .to_string()
                .starts_with(&self.beam_elements_group_number.to_string())
            {
                let mut beam_element_analysis_result_data = [0.0 as FEFloat; 19];

                let rotation_matrix_elements = self
                    .fem
                    .get_beam_rotation_matrix_elements(*element_number)
                    .map_err(JsValue::from)?;

                rotation_matrix_elements
                    .iter()
                    .enumerate()
                    .for_each(|(i, v)| beam_element_analysis_result_data[i] = *v);

                if element_analysis_result.len() != 10 {
                    return Err(JsValue::from(&format!(
                        "Solver: Incorrect number of result element loads for beam element {element_number}!",
                    )));
                }

                element_analysis_result
                    .iter()
                    .enumerate()
                    .for_each(|(i, (_, v))| beam_element_analysis_result_data[i + 9] = *v);

                beam_elements_analysis_result_data
                    .insert(*element_number, beam_element_analysis_result_data);
            } else if element_number
                .to_string()
                .starts_with(&self.plate_elements_group_number.to_string())
            {
                let mut plate_element_analysis_result_data = [0.0 as FEFloat; 17];

                let rotation_matrix_elements = self
                    .fem
                    .get_plate_rotation_matrix_elements(*element_number)
                    .map_err(JsValue::from)?;

                rotation_matrix_elements
                    .iter()
                    .enumerate()
                    .for_each(|(i, v)| plate_element_analysis_result_data[i] = *v);

                if element_analysis_result.len() != 8 {
                    return Err(JsValue::from(&format!(
                        "Solver: Incorrect number of result element loads for plate element {element_number}!",
                    )));
                }

                element_analysis_result
                    .iter()
                    .enumerate()
                    .for_each(|(i, (_, v))| plate_element_analysis_result_data[i + 9] = *v);

                plate_elements_analysis_result_data
                    .insert(*element_number, plate_element_analysis_result_data);
            } else {
                return Err(JsValue::from(&format!(
                    "Unknown group of finite element {element_number}!"
                )));
            }
        }

        let serializer = Serializer::json_compatible();
        let elements_analysis_result = json!({
            "elements_analysis_result": {
                "truss_elements": truss_elements_analysis_result_data,
                "beam_elements": beam_elements_analysis_result_data,
                "plate_elements": plate_elements_analysis_result_data,
            },
            "extreme_elements_loads": extreme_elements_loads,
        })
        .serialize(&serializer)
        .or(Err(JsValue::from(
            "Solver: Elements analysis result could not be composed for extraction!",
        )))?;

        Ok(elements_analysis_result)
    }
}
