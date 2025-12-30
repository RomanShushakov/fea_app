import { INIT_GLOBAL_MESH_SEED_VALUE } from "../consts/common_consts.js";


let _instance = null;


class Store {
    constructor() {
        if (!_instance) {
            this._points_shelf = new Map();
            this._lines_shelf = new Map();
            this._surfaces_shelf = new Map();
            this._materials_shelf = [];
            this._truss_sections_shelf = []; 
            this._beam_sections_shelf = [];
            this._plate_sections_shelf = [];
            this._properties_shelf = [];
            this._beam_sections_local_axis1_directions_shelf = [];
            this._concentrated_loads_shelf = new Map();
            this._point_boundary_conditions_shelf = new Map();
            this._global_mesh_seed_shelf = INIT_GLOBAL_MESH_SEED_VALUE;
            this._jobs_shelf = new Map();
            _instance = this;
        }
        return _instance;
    }

    addPoint(point) {
        this._points_shelf.set(point.number, {uid: point.uid, x: point.x, y: point.y, z: point.z});
    }

    updatePoint(point) {
        this._points_shelf.set(point.number, {uid: point.uid, x: point.x, y: point.y, z: point.z});
    }

    deletePoint(point) {
        this._points_shelf.delete(point.number);
    }

    get points_shelf() {
        return this._points_shelf;
    }

    addLine(line) {
        this._lines_shelf.set(line.number, { 
            uid: line.uid, 
            point_1_number: line.point_1_number, 
            point_2_number: line.point_2_number, 
            optional_property: line.optional_property,
            optional_local_axis_1_direction: line.optional_local_axis_1_direction,
            optional_transformed_local_axis_1_direction: line.optional_transformed_local_axis_1_direction,
            optional_mesh_seed: line.optional_mesh_seed,
            optional_uniformly_distributed_line_load: line.optional_uniformly_distributed_line_load,
        });
    }

    updateLine(line) {
        this._lines_shelf.set(line.number, { 
            uid: line.uid, 
            point_1_number: line.point_1_number, 
            point_2_number: line.point_2_number,
            optional_property: line.optional_property,
            optional_local_axis_1_direction: line.optional_local_axis_1_direction,
            optional_transformed_local_axis_1_direction: line.optional_transformed_local_axis_1_direction,
            optional_mesh_seed: line.optional_mesh_seed,
            optional_uniformly_distributed_line_load: line.optional_uniformly_distributed_line_load,
        });
    }

    deleteLine(line) {
        this._lines_shelf.delete(line.number);
    }

    get lines_shelf() {
        return this._lines_shelf;
    }

    addSurface(surface) {
        this._surfaces_shelf.set(surface.number, { 
            uid: surface.uid, 
            point_1_number: surface.point_1_number, 
            point_2_number: surface.point_2_number,
            point_3_number: surface.point_3_number, 
            point_4_number: surface.point_4_number, 
            normal: surface.normal, 
            optional_property: surface.optional_property,
            optional_mesh_seed: surface.optional_mesh_seed,
            optional_uniformly_distributed_surface_load: surface.optional_uniformly_distributed_surface_load,
        });
    }

    updateSurface(surface) {
        this._surfaces_shelf.set(surface.number, { 
            uid: surface.uid, 
            point_1_number: surface.point_1_number, 
            point_2_number: surface.point_2_number,
            point_3_number: surface.point_3_number, 
            point_4_number: surface.point_4_number,
            normal: surface.normal, 
            optional_property: surface.optional_property,
            optional_mesh_seed: surface.optional_mesh_seed,
            optional_uniformly_distributed_surface_load: surface.optional_uniformly_distributed_surface_load,
        });
    }

    deleteSurface(surface) {
        this._surfaces_shelf.delete(surface.number);
    }

    get surfaces_shelf() {
        return this._surfaces_shelf;
    }

    addMaterial(material) {
        this._materials_shelf.push(material);
        this._materials_shelf.sort((a, b) => a.name - b.name);
    }

    updateMaterial(material) {
        let materialOnShelf = this._materials_shelf.find(existedMaterial => existedMaterial.name == material.name);
        materialOnShelf.young_modulus = material.young_modulus;
        materialOnShelf.poisson_ratio = material.poisson_ratio;
    }

    deleteMaterial(material) {
        let materialIndexOnShelf = this._materials_shelf.findIndex(existedMaterial => existedMaterial.name == material.name);
        this._materials_shelf.splice(materialIndexOnShelf, 1);
        this._materials_shelf.sort((a, b) => a.name - b.name);
    }

    get materials_shelf() {
        return this._materials_shelf;
    }

    addTrussSection(trussSection) {
        this._truss_sections_shelf.push(trussSection);
        this._truss_sections_shelf.sort((a, b) => a.name - b.name);
    }

    updateTrussSection(trussSection) {
        let trussSectionOnShelf = this._truss_sections_shelf
            .find(existedTrussSection => existedTrussSection.name == trussSection.name);
        trussSectionOnShelf.area = trussSection.area;
        trussSectionOnShelf.area2 = trussSection.area2;
    }

    deleteTrussSection(trussSection) {
        let trussSectionIndexOnShels = this._truss_sections_shelf
            .findIndex(existedTrussSection => existedTrussSection.name == trussSection.name);
        this._truss_sections_shelf.splice(trussSectionIndexOnShels, 1);
        this._truss_sections_shelf.sort((a, b) => a.name - b.name);
    }

    get truss_sections_shelf() {
        return this._truss_sections_shelf;
    }

    addBeamSection(beamSection) {
        this._beam_sections_shelf.push(beamSection);
        this._beam_sections_shelf.sort((a, b) => a.name - b.name);
    }

    updateBeamSection(beamSection) {
        let beamSectionOnShelf = this._beam_sections_shelf
            .find(existedBeamSection => existedBeamSection.name == beamSection.name);
        beamSectionOnShelf.area = beamSection.area;
        beamSectionOnShelf.i11 = beamSection.i11;
        beamSectionOnShelf.i22 = beamSection.i22;
        beamSectionOnShelf.i12 = beamSection.i12;
        beamSectionOnShelf.it = beamSection.it;
        beamSectionOnShelf.shear_factor = beamSection.shear_factor;
    }

    deleteBeamSection(beamSection) {
        let beamSectionOnShelf = this._beam_sections_shelf
            .findIndex(existedBeamSection => existedBeamSection.name == beamSection.name);
        this._beam_sections_shelf.splice(beamSectionOnShelf, 1);
        this._beam_sections_shelf.sort((a, b) => a.name - b.name);
    }

    get beam_sections_shelf() {
        return this._beam_sections_shelf;
    }

    addPlateSection(plateSection) {
        this._plate_sections_shelf.push(plateSection);
        this._plate_sections_shelf.sort((a, b) => a.name - b.name);
    }

    updatePlateSection(plateSection) {
        let plateSectionOnShelf = this._plate_sections_shelf
            .find(existedPlateSection => existedPlateSection.name == plateSection.name);
        plateSectionOnShelf.thickness = plateSection.thickness;
        plateSectionOnShelf.shear_factor = plateSection.shear_factor;
    }

    deletePlateSection(plateSection) {
        let plateSectionOnShelf = this._plate_sections_shelf
            .findIndex(existedPlateSection => existedPlateSection.name == plateSection.name);
        this._plate_sections_shelf.splice(plateSectionOnShelf, 1);
        this._plate_sections_shelf.sort((a, b) => a.name - b.name);
    }

    get plate_sections_shelf() {
        return this._plate_sections_shelf;
    }

    addProperties(properties) {
        this._properties_shelf.push(properties);
        this._properties_shelf.sort((a, b) => a.name - b.name);
    }

    updateProperties(properties) {
        let propertiesOnShelf = this._properties_shelf
            .find(existedProperties => existedProperties.name == properties.name);
        propertiesOnShelf.material_name = properties.material_name;
        propertiesOnShelf.cross_section_name = properties.cross_section_name;
        propertiesOnShelf.cross_section_type = properties.cross_section_type;
    }

    deleteProperties(properties) {
        let propertiesOnShelf = this._properties_shelf
            .findIndex(existedProperties => existedProperties.name == properties.name);
        this._properties_shelf.splice(propertiesOnShelf, 1);
        this._properties_shelf.sort((a, b) => a.name - b.name);
    }

    get properties_shelf() {
        return this._properties_shelf;
    }

    getLineNumbersWithProperty(propertyName) {
        let lineNumbers = Array();
        this.lines_shelf.forEach((line, number) => {
            if (line.optional_property) {
                if (line.optional_property.name === propertyName) {
                    lineNumbers.push(parseInt(number));
                }
            }
        });
        return lineNumbers;
    }

    getLineUidByNumber(lineNumber) {
        for (const [number, line] of this.lines_shelf.entries()) {
            if (number === lineNumber) {
                return line.uid;
            }
        }
    }

    getLineNumbersWithLocalAxis1Direction(localAxis1Direction) {
        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
        let lineNumbers = Array();
        this.lines_shelf.forEach((line, number) => {
            if (line.optional_local_axis_1_direction) {
                if (equals(line.optional_local_axis_1_direction, localAxis1Direction)) {
                    lineNumbers.push(parseInt(number));
                }
            }
        });
        return lineNumbers;
    }

    getSurfaceNumbersWithProperty(propertyName) {
        let surfaceNumbers = Array();
        this.surfaces_shelf.forEach((surface, number) => {
            if (surface.optional_property) {
                if (surface.optional_property.name === propertyName) {
                    surfaceNumbers.push(parseInt(number));
                }
            }
        });
        return surfaceNumbers;
    }

    getSurfaceUidByNumber(surfaceNumber) {
        for (const [number, surface] of this.surfaces_shelf.entries()) {
            if (number === surfaceNumber) {
                return surface.uid;
            }
        }
    }

    addBeamSectionLocalAxis1Direction(beamSectionLocalAxis1DirectionData) {
        this._beam_sections_local_axis1_directions_shelf.push(beamSectionLocalAxis1DirectionData);
        this._beam_sections_local_axis1_directions_shelf.sort((a, b) => a - b);
    }

    deleteBeamSectionLocalAxis1Direction(beamSectionLocalAxis1DirectionData) {
        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
        let beamSectionLocalAxis1DirectionIndexOnShelf = this._beam_sections_local_axis1_directions_shelf
            .findIndex(existedbeamSectionLocalAxis1Direction => equals(existedbeamSectionLocalAxis1Direction,
                beamSectionLocalAxis1DirectionData));
        this._beam_sections_local_axis1_directions_shelf.splice(beamSectionLocalAxis1DirectionIndexOnShelf, 1);
        this._beam_sections_local_axis1_directions_shelf.sort((a, b) => a - b);
    }

    get beam_sections_local_axis1_directions_shelf() {
        return this._beam_sections_local_axis1_directions_shelf;
    }

    addConcentratedLoad(concentratedLoad) {
        this._concentrated_loads_shelf.set(concentratedLoad.point_number, 
            {
                uid: concentratedLoad.uid, fx: concentratedLoad.fx, fy: concentratedLoad.fy, 
                fz: concentratedLoad.fz, mx: concentratedLoad.mx, my: concentratedLoad.my, 
                mz: concentratedLoad.mz,
            });
    }

    updateConcentratedLoad(concentratedLoad) {
        this._concentrated_loads_shelf.set(concentratedLoad.point_number, 
            {
                uid: concentratedLoad.uid, fx: concentratedLoad.fx, fy: concentratedLoad.fy, 
                fz: concentratedLoad.fz, mx: concentratedLoad.mx, my: concentratedLoad.my, 
                mz: concentratedLoad.mz,
            });
    }

    deleteConcentratedLoad(concentratedLoad) {
        this._concentrated_loads_shelf.delete(concentratedLoad.point_number);
    }

    get concentrated_loads_shelf() {
        return this._concentrated_loads_shelf;
    }

    addPointBoundaryCondition(pointBoundaryCondition) {
        this._point_boundary_conditions_shelf.set(pointBoundaryCondition.point_number, 
            {
                uid: pointBoundaryCondition.uid,
                optional_ux: pointBoundaryCondition.optional_ux, optional_uy: pointBoundaryCondition.optional_uy, 
                optional_uz: pointBoundaryCondition.optional_uz, optional_rx: pointBoundaryCondition.optional_rx, 
                optional_ry: pointBoundaryCondition.optional_ry, optional_rz: pointBoundaryCondition.optional_rz,
            });
    }

    updatePointBoundaryCondition(pointBoundaryCondition) {
        this._point_boundary_conditions_shelf.set(pointBoundaryCondition.point_number, 
            {
                uid: pointBoundaryCondition.uid,
                optional_ux: pointBoundaryCondition.optional_ux, optional_uy: pointBoundaryCondition.optional_uy, 
                optional_uz: pointBoundaryCondition.optional_uz, optional_rx: pointBoundaryCondition.optional_rx, 
                optional_ry: pointBoundaryCondition.optional_ry, optional_rz: pointBoundaryCondition.optional_rz,
            });
    }

    deletePointBoundaryCondition(pointBoundaryCondition) {
        this._point_boundary_conditions_shelf.delete(pointBoundaryCondition.point_number);
    }

    get point_boundary_conditions_shelf() {
        return this._point_boundary_conditions_shelf;
    }

    updateGlobalMeshSeedValue(globalMeshSeedValue) {
        this._global_mesh_seed_shelf = globalMeshSeedValue;
    }

    get global_mesh_seed() {
        return this._global_mesh_seed_shelf;
    }

    addJob(jobName, mesh) {
        const job = { mesh }
        this._jobs_shelf.set(jobName, job);
    }

    deleteJob(jobName) {
        this._jobs_shelf.delete(jobName);
    }

    get jobs_shelf() {
        return this._jobs_shelf;
    }

    reset() {
        this._points_shelf = new Map();
        this._lines_shelf = new Map();
        this._surfaces_shelf = new Map();
        this._materials_shelf = [];
        this._truss_sections_shelf = []; 
        this._beam_sections_shelf = [];
        this._plate_sections_shelf = [];
        this._properties_shelf = [];
        this._beam_sections_local_axis1_directions_shelf = [];
        this._concentrated_loads_shelf = new Map();
        this._point_boundary_conditions_shelf = new Map();
        this._global_mesh_seed_shelf = INIT_GLOBAL_MESH_SEED_VALUE;  
    }
}

export default Store;
