use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::{Renderer, enums::{SceneState, ResultPlot}};


#[wasm_bindgen]
impl Renderer
{
    pub fn plot_displacements(&mut self, magnitude: f32) -> Result<(), JsValue>
    {
        if magnitude <= 0f32
        {
            let error_message = "Renderer: Displacements magnitude should be greater than zero!";
            return Err(JsValue::from(error_message));
        }

        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Displacements could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::Displacements(magnitude));
            }
        }

        Ok(())
    }


    pub fn plot_global_forces(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Global forces could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::GlobalForces);
            }
        }

        Ok(())
    }


    pub fn plot_global_moments(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Global moments could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::GlobalMoments);
            }
        }

        Ok(())
    }


    pub fn plot_elements_forces_r(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements forces r could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::ForceR);
            }
        }

        Ok(())
    }


    pub fn plot_elements_forces_s(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements forces s could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::ForceS);
            }
        }

        Ok(())
    }


    pub fn plot_elements_forces_t(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements forces t could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::ForceT);
            }
        }

        Ok(())
    }


    pub fn plot_elements_moments_r(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements moments r could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::MomentR);
            }
        }

        Ok(())
    }


    pub fn plot_elements_moments_s(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements moments s could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::MomentS);
            }
        }

        Ok(())
    }


    pub fn plot_elements_moments_t(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements moments t could not be drawn at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::MomentT);
            }
        }

        Ok(())
    }


    pub fn plot_elements_mem_forces_r(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements membrane forces r could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::MemForceR);
            }
        }

        Ok(())
    }


    pub fn plot_elements_mem_forces_s(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements membrane forces s could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::MemForceS);
            }
        }

        Ok(())
    }


    pub fn plot_elements_mem_forces_r_s(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements membrane forces r s could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::MemForceRS);
            }
        }

        Ok(())
    }


    pub fn plot_elements_bend_moment_r(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements bending moments r could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::BendMomentR);
            }
        }

        Ok(())
    }


    pub fn plot_elements_bend_moment_s(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements bending moments s could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::BendMomentS);
            }
        }

        Ok(())
    }


    pub fn plot_elements_bend_moment_r_s(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements bending moments r s could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::BendMomentRS);
            }
        }

        Ok(())
    }


    pub fn plot_elements_shear_force_r_t(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements shear forces r t could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::ShearForceRT);
            }
        }

        Ok(())
    }


    pub fn plot_elements_shear_force_s_t(&mut self) -> Result<(), JsValue>
    {
        match self.scene.get_mut_ref_scene_state()
        {
            SceneState::Preprocessor => 
            {
                return Err(JsValue::from("Renderer: Elements shear forces s t could not be drawn \
                    at preprocessor state!"));
            },
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                *optional_result_plot = Some(ResultPlot::ShearForceST);
            }
        }

        Ok(())
    }
}
