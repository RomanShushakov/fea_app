import { initializeRenderer } from "../wasm_modules_initialization/renderer_initialization.js";

import { LineObjectType, SurfaceObjectType, MeshSeedType } from "../wasm/renderer/renderer.js";

import { RENDERER_LOADED_MESSAGE_HEADER } from "../consts/fea_app_consts.js";


class FeaRenderer extends HTMLElement {
    constructor() {
        super();

        this.props = {
            canvasWidth: null,
            canvasHeight: null,
        };

        this.state = {
            isRendererLoaded: false,    // load status of wasm module "renderer";
            canvasText: null,
            canvasGL: null,
            renderer: null,
            animationId: null,
            renderLoop: null,
            isPaused: true,
            isRotate: false,
            isPan: false,
            isZoom: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: flex;
            }

            .wrapper {
                display: flex;
                flex-direction: column;
                border-left: 0.1rem solid #5c687a;
                border-right: 0.1rem solid #5c687a;
                border-bottom: 0.1rem solid #5c687a;
            }
            
            .renderer-container {
                position: relative;
                border-bottom: 0.1rem solid #5c687a;
            }
            
            .renderer-canvas-text {
                background-color: transparent;
                position: absolute;
                left: 0px;
                top: 0px;
                z-index: 10;
            }
            
            .renderer-canvas-gl {
                vertical-align: top;
            }

            .object-info {
                margin: 0rem;
                padding: 0rem;
                height: 3rem;
            }

            .object-info-field {
                margin: 0rem;
                padding: 0rem;
                resize: none;
                color: #6c6c6d;
                background-color : #212933; 
                border: 0rem;
                width: 99.5%;
                height: 100%;
                outline: 0;
            }

            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            ::-webkit-scrollbar-track {
                background: var(--renderer-menu-content-scrollbar-track-color);
            }

            ::-webkit-scrollbar-thumb {
                background: var(--renderer-menu-content-scrollbar-thumb-color);
            }

            ::-webkit-scrollbar-thumb:hover {
                background: var(--renderer-menu-content-scrollbar-thumb-hover-color);
            }
        </style>
        <div class="wrapper">
            <div class="renderer-container">
                <canvas class="renderer-canvas-text" oncontextmenu="return false;"></canvas>
                <canvas class="renderer-canvas-gl" oncontextmenu="return false;"></canvas>
            </div>
            <div class="object-info">
                <textarea class="object-info-field"></textarea>
            </div>
        </div>
        `;

        window.addEventListener("keyup", (event) => this.onKeyUp(event));
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mousemove", 
            (event) => this.onMouseMove(event));
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mouseleave", 
            () => this.onMouseLeave());
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mousedown", 
            (event) => this.onMouseDown(event));
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("mouseup", 
            (event) => this.onMouseUp(event));
        this.shadowRoot.querySelector(".renderer-canvas-text").addEventListener("wheel", 
            (event) => this.onMouseWheel(event));
    }

    set addPointToRenderer(point) {
        try {
            this.state.renderer.add_point(point.number, point.uid, point.x, point.y, point.z);
            if (this.state.isPaused === true) {
                this.state.renderer.tick();
            }
        } catch (error) {
            throw error;
        }
    }

    set updatePointInRenderer(point) {
        try {
            this.state.renderer.update_point(point.number, point.uid, point.x, point.y, point.z);
            if (this.state.isPaused === true) {
                this.state.renderer.tick();
            }
        } catch (error) {
            throw error;
        }
        
    }

    set deletePointFromRenderer(point) {
        try {
            this.state.renderer.delete_point(point.number, point.uid);
            if (this.state.isPaused === true) {
                this.state.renderer.tick();
            }
        } catch (error) {
            throw error;
        }
    }

    defineLineObjectType(line) {
        let lineObjectType = LineObjectType.LineDefault;
        if (line.optional_property) {
            const crossSectionType = Object.keys(line.optional_property.property.cross_section)[0];
            switch (crossSectionType) {
                case "Truss":
                    lineObjectType = LineObjectType.LineTruss;
                    break;
                case "Beam":
                    lineObjectType = LineObjectType.LineBeam;
                    break;
                default:
                    console.log(`Unknown line object type: ${crossSectionType}`);
            }
        }
        return lineObjectType;
    }

    defineMeshSeed(obj) {
        let objMeshSeed = { 
            type: MeshSeedType.None, 
            global_mesh_seed_value: null,
            line_mesh_seed_value: null, 
            edge_1_3_mesh_seed_value: null,
            edge_2_4_mesh_seed_value: null,
        };
        if (obj.optional_mesh_seed) {
            if (obj.optional_mesh_seed.hasOwnProperty("Global")) {
                objMeshSeed.type = MeshSeedType.Global;
                objMeshSeed.global_mesh_seed_value = obj.optional_mesh_seed["Global"];
            }
            if (obj.optional_mesh_seed.hasOwnProperty("Line")) {
                objMeshSeed.type = MeshSeedType.Local;
                objMeshSeed.line_mesh_seed_value = obj.optional_mesh_seed["Line"];
            }
            if (obj.optional_mesh_seed.hasOwnProperty("Surface")) {
                objMeshSeed.type = MeshSeedType.Local;
                [objMeshSeed.edge_1_3_mesh_seed_value, objMeshSeed.edge_2_4_mesh_seed_value] = 
                    obj.optional_mesh_seed["Surface"];
            }
        }
        return objMeshSeed;
    }

    defineUniformlyDistributedLineLoadData(line) {
        let [uid, components] = [null, null];
        if (line.optional_uniformly_distributed_line_load) {
            uid = line.optional_uniformly_distributed_line_load[0];
            components = line.optional_uniformly_distributed_line_load.slice(1);
        }
        return [uid, components];
    }

    set addLineToRenderer(line) {
        try {
            const lineObjectType = this.defineLineObjectType(line);
            const meshSeed = this.defineMeshSeed(line);
            const meshSeedValue = meshSeed.type === MeshSeedType.Local ? meshSeed.line_mesh_seed_value : 
                meshSeed.global_mesh_seed_value;
            const [uniformlyDistributedLineLoadUID, uniformlyDistributedLineLoadComponents] = 
                this.defineUniformlyDistributedLineLoadData(line);
            this.state.renderer.add_line(line.number, line.uid, line.point_1_number, line.point_2_number,
                line.optional_transformed_local_axis_1_direction, lineObjectType, meshSeedValue, meshSeed.type,
                uniformlyDistributedLineLoadUID, uniformlyDistributedLineLoadComponents);
            if (this.state.isPaused === true) {
                this.state.renderer.tick();
            }
        } catch (error) {
            throw error;
        }
    }

    set updateLineInRenderer(line) {
        try {
            const lineObjectType = this.defineLineObjectType(line);
            const meshSeed = this.defineMeshSeed(line);
            const meshSeedValue = meshSeed.type === MeshSeedType.Local ? meshSeed.line_mesh_seed_value : 
                meshSeed.global_mesh_seed_value;
            const [uniformlyDistributedLineLoadUID, uniformlyDistributedLineLoadComponents] = 
                this.defineUniformlyDistributedLineLoadData(line);
            this.state.renderer.update_line(line.number, line.uid, line.point_1_number, line.point_2_number,
                line.optional_transformed_local_axis_1_direction, lineObjectType, meshSeedValue, meshSeed.type,
                uniformlyDistributedLineLoadUID, uniformlyDistributedLineLoadComponents);
            if (this.state.isPaused === true) {
                this.state.renderer.tick();
            }
        } catch (error) {
            throw error;
        }
    }

    set deleteLineFromRenderer(line) {
        try {
            this.state.renderer.delete_line(line.number, line.uid);
            if (this.state.isPaused === true) {
                this.state.renderer.tick();
            }
        } catch (error) {
            throw error;
        }
    }

    defineSurfaceObjectType(surface) {
        let surfaceObjectType = SurfaceObjectType.SurfaceDefault;
        if (surface.optional_property) {
            surfaceObjectType = SurfaceObjectType.SurfacePlate;
        }
        return surfaceObjectType;
    }

    defineUniformlyDistributedSurfaceLoadData(surface) {
        let [uid, components] = [null, null];
        if (surface.optional_uniformly_distributed_surface_load) {
            uid = surface.optional_uniformly_distributed_surface_load[0];
            components = surface.optional_uniformly_distributed_surface_load.slice(1);
        }
        return [uid, components];
    }

    set addSurfaceToRenderer(surface) {
        try {
            const surfaceObjectType = this.defineSurfaceObjectType(surface);
            const meshSeed = this.defineMeshSeed(surface);
            let meshSeedValues = meshSeed.type === MeshSeedType.Local ? 
                [meshSeed.edge_1_3_mesh_seed_value, meshSeed.edge_2_4_mesh_seed_value] : 
                [meshSeed.global_mesh_seed_value, meshSeed.global_mesh_seed_value];
            if (meshSeed.type === MeshSeedType.None) {
                meshSeedValues = null;
            }
            const [uniformlyDistributedSurfaceLoadUID, uniformlyDistributedSurfaceLoadComponents] = 
                this.defineUniformlyDistributedSurfaceLoadData(surface);
            this.state.renderer.add_surface(surface.number, surface.uid, surface.point_1_number, surface.point_2_number, 
                surface.point_3_number, surface.point_4_number, surface.normal, surfaceObjectType, meshSeedValues, 
                meshSeed.type, uniformlyDistributedSurfaceLoadUID, uniformlyDistributedSurfaceLoadComponents);
        } catch (error) {
            throw error;
        }
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateSurfaceInRenderer(surface) {
        try {
            const surfaceObjectType = this.defineSurfaceObjectType(surface);
            const meshSeed = this.defineMeshSeed(surface);
            let meshSeedValues = meshSeed.type === MeshSeedType.Local ? 
                [meshSeed.edge_1_3_mesh_seed_value, meshSeed.edge_2_4_mesh_seed_value] : 
                [meshSeed.global_mesh_seed_value, meshSeed.global_mesh_seed_value];
            if (meshSeed.type === MeshSeedType.None) {
                meshSeedValues = null;
            }
            const [uniformlyDistributedSurfaceLoadUID, uniformlyDistributedSurfaceLoadComponents] = 
                this.defineUniformlyDistributedSurfaceLoadData(surface);
            this.state.renderer.update_surface(surface.number, surface.uid, surface.point_1_number, 
                surface.point_2_number, surface.point_3_number, surface.point_4_number, surface.normal, 
                surfaceObjectType, meshSeedValues, meshSeed.type, uniformlyDistributedSurfaceLoadUID,
                uniformlyDistributedSurfaceLoadComponents);
        } catch (error) {
            throw error;
        }
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set deleteSurfaceFromRenderer(surface) {
        try {
            this.state.renderer.delete_surface(surface.number, surface.uid);
        } catch (error) {
            throw error;
        }
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set addConcentratedLoadToRenderer(concentratedLoad) {
        this.state.renderer.add_concentrated_load(
            concentratedLoad.point_number, concentratedLoad.uid,
            concentratedLoad.fx, concentratedLoad.fy, concentratedLoad.fz,
            concentratedLoad.mx, concentratedLoad.my, concentratedLoad.mz);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateConcentratedLoadInRenderer(concentratedLoad) {
        this.state.renderer.update_concentrated_load(
            concentratedLoad.point_number, concentratedLoad.uid,
            concentratedLoad.fx, concentratedLoad.fy, concentratedLoad.fz,
            concentratedLoad.mx, concentratedLoad.my, concentratedLoad.mz);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set deleteConcentratedLoadFromRenderer(concentratedLoad) {
        this.state.renderer.delete_concentrated_load(concentratedLoad.point_number, concentratedLoad.uid);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set addPointBoundaryConditionToRenderer(pointBoundaryCondition) {
        this.state.renderer.add_point_boundary_condition(pointBoundaryCondition.point_number, 
            pointBoundaryCondition.uid, pointBoundaryCondition.optional_ux, pointBoundaryCondition.optional_uy,
            pointBoundaryCondition.optional_uz, pointBoundaryCondition.optional_rx, pointBoundaryCondition.optional_ry,
            pointBoundaryCondition.optional_rz);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updatePointBoundaryConditionInRenderer(pointBoundaryCondition) {
        this.state.renderer.update_point_boundary_condition(pointBoundaryCondition.point_number, 
            pointBoundaryCondition.uid, pointBoundaryCondition.optional_ux, pointBoundaryCondition.optional_uy,
            pointBoundaryCondition.optional_uz, pointBoundaryCondition.optional_rx, pointBoundaryCondition.optional_ry,
            pointBoundaryCondition.optional_rz);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set deletePointBoundaryConditionFromRenderer(pointBoundaryCondition) {
        this.state.renderer.delete_point_boundary_condition(pointBoundaryCondition.point_number, 
            pointBoundaryCondition.uid);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set addAnalysisResultToRenderer(analysisResultData) {
        this.state.renderer.add_analysis_result(analysisResultData);
        if (this.state.isPaused === true)
        {
            this.state.renderer.tick();
        }
    }

    set objectInfo(objectInfo) {
        let info = JSON.stringify(objectInfo).replaceAll('"', '').replaceAll("\\n", "").replace(/ +(?= )/g, "");
        this.shadowRoot.querySelector(".object-info-field").innerHTML = info;
    }

    set canvasSize(size) {
        this.props.canvasWidth = size.width;
        this.props.canvasHeight = size.height;
        this.setCanvasSize();
    }

    set updatePointVisibility(isPointVisible) {
        this.state.renderer.set_point_visibility(isPointVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateLineVisibility(isLineVisible) {
        this.state.renderer.set_line_visibility(isLineVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateSurfaceVisibility(isSurfaceVisible) {
        this.state.renderer.set_surface_visibility(isSurfaceVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateSurfaceEdges13Visibility(isSurfaceEdges13Visible) {
        this.state.renderer.set_surface_edges_1_3_visibility(isSurfaceEdges13Visible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateSurfaceEdges24Visibility(isSurfaceEdges24Visible) {
        this.state.renderer.set_surface_edges_2_4_visibility(isSurfaceEdges24Visible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateBeamSectionOrientationVisibility(isBeamSectionOrientationVisible) {
        this.state.renderer.set_beam_section_orientation_visibility(isBeamSectionOrientationVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateSurfaceNormalVisibility(isSurfaceNormalVisible) {
        this.state.renderer.set_surface_normal_visibility(isSurfaceNormalVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateLoadVisibility(isLoadVisible) {
        this.state.renderer.set_load_visibility(isLoadVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateBoundaryConditionVisibility(isBoundaryConditionVisible) {
        this.state.renderer.set_boundary_condition_visibility(isBoundaryConditionVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateMeshSeedVisibility(isMeshSeedVisible) {
        this.state.renderer.set_mesh_seed_visibility(isMeshSeedVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateNodeVisibility(isNodeVisible) {
        this.state.renderer.set_node_visibility(isNodeVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateTrussElementVisibility(isTrussElementVisible) {
        this.state.renderer.set_truss_element_visibility(isTrussElementVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateBeamElementVisibility(isBeamElementVisible) {
        this.state.renderer.set_beam_element_visibility(isBeamElementVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updatePlateElementVisibility(isPlateElementVisible) {
        this.state.renderer.set_plate_element_visibility(isPlateElementVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    set updateLocalAxesVisibility(isLocalAxesVisible) {
        this.state.renderer.set_local_axes_visibility(isLocalAxesVisible);
        if (this.state.isPaused === true) {
            this.state.renderer.tick();
        }
    }

    autoFit() {
        if (this.state.renderer !== null) {
            this.state.renderer.auto_fit();
            this.state.renderer.tick();
        }
    }

    set selectedView(view) {
        switch (view) {
            case "planeXY":
                this.state.renderer.set_theta(0.0);
                this.state.renderer.set_phi(0.0);
                if (this.state.isPaused === true) {
                    this.state.renderer.tick();
                }
                break;
            case "planeZY":
                this.state.renderer.set_theta(90.0 * Math.PI / 180.0);
                this.state.renderer.set_phi(0.0);
                if (this.state.isPaused === true) {
                    this.state.renderer.tick();
                }
                break;
            case "planeXZ":
                this.state.renderer.set_theta(0.0);
                this.state.renderer.set_phi(-90.0 * Math.PI / 180.0);
                if (this.state.isPaused === true) {
                    this.state.renderer.tick();
                }
                break;
            case "isometric":
                this.state.renderer.set_theta(-45.0 * Math.PI / 180.0);
                this.state.renderer.set_phi(35.264 * Math.PI / 180.0);
                if (this.state.isPaused === true) {
                    this.state.renderer.tick();
                }
                break;
        }
    }

    // set previewSelectedLineNumbers(selectedLineNumbersObject) {
    //     try {
    //         this.state.renderer.preview_selected_line_objects(selectedLineNumbersObject, LineObjectType.Line);
    //     } catch (error) {
    //         throw error;   
    //     }
    //     this.state.renderer.tick();
    // }

    set previewSelectedObjects(uids) {
        try {
            this.state.renderer.preview_selected_objects(uids, () => this.dropSelection());
        } catch (error) {
            throw error;   
        }
        this.state.renderer.tick();
    }

    // set previewBeamSectionOrientation(beamSectionOrientationObject) {
    //     try {
    //         this.state.renderer.preview_beam_section_orientation(beamSectionOrientationObject, LineObjectType.Line);
    //     } catch (error) {
    //         throw error;
    //     }
    //     this.state.renderer.tick();
    // }

    activatePreprocessorState() {
        this.state.renderer.activate_preprocessor_state();
        this.state.renderer.tick();
    }

    activatePostprocessorState(job) {
        try {
            this.state.renderer.activate_postprocessor_state(job);
        } catch (error) {
            throw error;
        }
        this.state.renderer.tick();
    }

    set plotDisplacements(plotDisplacementsData) {
        try {
            this.state.renderer.plot_displacements(plotDisplacementsData.magnitude);
        } catch (error) {
            throw error;
        }
        this.state.renderer.tick();
    }

    plotGlobalForces() {
        try {
            this.state.renderer.plot_global_forces();
        } catch (error) {
            throw error;
        }
        this.state.renderer.tick();
    }

    plotGlobalMoments() {
        try {
            this.state.renderer.plot_global_moments();
        } catch (error) {
            throw error;
        }
        this.state.renderer.tick();
    }

    set plotElementsLoads(selectedElementLoadComponentData) {
        try {
            switch (selectedElementLoadComponentData.selected_element_load_component) {
                case "elforce_r":
                    this.state.renderer.plot_elements_forces_r();
                    break;
                case "elforce_s":
                    this.state.renderer.plot_elements_forces_s();
                    break;
                case "elforce_t":
                    this.state.renderer.plot_elements_forces_t();
                    break;
                case "elmoment_r":
                    this.state.renderer.plot_elements_moments_r();
                    break;
                case "elmoment_s":
                    this.state.renderer.plot_elements_moments_s();
                    break;
                case "elmoment_t":
                    this.state.renderer.plot_elements_moments_t();
                    break;
                case "elmemforce_r":
                    this.state.renderer.plot_elements_mem_forces_r();
                    break;
                case "elmemforce_s":
                    this.state.renderer.plot_elements_mem_forces_s();
                    break;
                case "elmemforce_r_s":
                    this.state.renderer.plot_elements_mem_forces_r_s();
                    break;
                case "elbendmoment_r":
                    this.state.renderer.plot_elements_bend_moment_r();
                    break;
                case "elbendmoment_s":
                    this.state.renderer.plot_elements_bend_moment_s();
                    break;
                case "elbendmoment_r_s":
                    this.state.renderer.plot_elements_bend_moment_r_s();
                    break;
                case "elshearforce_r_t":
                    this.state.renderer.plot_elements_shear_force_r_t();
                    break;
                case "elshearforce_s_t":
                    this.state.renderer.plot_elements_shear_force_s_t();
                    break;
            }
        } catch (error) {
            throw error;
        }
        this.state.renderer.tick();
    }

    resetScene() {
        this.state.renderer.reset_scene();
        this.state.renderer.tick();
    }

    async connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.state.canvasText = this.shadowRoot.querySelector(".renderer-canvas-text");
        this.state.canvasGL = this.shadowRoot.querySelector(".renderer-canvas-gl");
        this.props.canvasWidth = window.innerWidth;
        this.props.canvasHeight = window.innerHeight;
        this.state.canvasText.width = this.props.canvasWidth;
        this.state.canvasText.height = this.props.canvasHeight;
        this.state.canvasGL.width = this.props.canvasWidth;
        this.state.canvasGL.height = this.props.canvasHeight;
        this.state.renderer = await initializeRenderer(this.state.canvasText, this.state.canvasGL);
        this.state.isRendererLoaded = true;

        document.querySelector("fea-app").dispatchEvent(new CustomEvent(RENDERER_LOADED_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));
        
        this.state.renderLoop = () => {
            this.state.renderer.tick();
            this.state.animationId = requestAnimationFrame(this.state.renderLoop);
        };
        this.setCanvasSize();
    }

    static get observedAttributes() {
        return [];
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
    }

    play() {
        this.state.renderLoop();
    }

    pause() {
        cancelAnimationFrame(this.state.animationId);
        this.state.animationId = null;
    }

    setCanvasSize() {
        if (this.state.renderer !== null) {
            this.state.renderer.set_canvas_size(this.props.canvasWidth, this.props.canvasHeight);
            this.state.renderer.tick();
        }
    }

    onKeyUp(event) {
        if (event.key === "Control" || event.key === "Alt") {
            this.state.isRotate = false;
            this.state.isPan = false;
            this.state.isZoom = false;
        }
    }

    onMouseMove(event) {
        const frame = () => {
            if (this.state.isRendererLoaded === true) {
                clearInterval(id);
                if (this.state.isPaused === true) {
                    this.play();
                    this.state.isPaused = false;
                }
                const mouseX = event.clientX;
                const mouseY = event.clientY;
                const boundingRect = this.state.canvasGL.getBoundingClientRect();
                const x = mouseX - boundingRect.left;
                const y = boundingRect.bottom - mouseY;
                this.state.renderer.set_cursor_coordinates(x, y);
                if (this.state.isRotate === true) {
                    const dTheta = event.movementX * 2.0 * Math.PI / this.props.canvasWidth;
                    this.state.renderer.increment_theta(dTheta);
                    const dPhi = event.movementY * 2.0 * Math.PI / this.props.canvasHeight;
                    this.state.renderer.increment_phi(dPhi);
                }
                if (this.state.isPan === true) {
                    const dx = event.movementX / Math.max(this.props.canvasWidth, this.props.canvasHeight);
                    this.state.renderer.increment_dx(dx);
                    const dy =  -event.movementY / Math.max(this.props.canvasWidth, this.props.canvasHeight);
                    this.state.renderer.increment_dy(dy);
                }
                if (this.state.isZoom === true) {
                    const dScale = this.state.renderer.get_d_scale() + 
                        event.movementX / this.props.canvasWidth + 
                        event.movementY / this.props.canvasHeight;
                    if (1.0 + dScale > 50.0) {
                        this.state.renderer.set_d_scale(48.95);
                    } else if (1.0 + dScale < 0.0) {
                        this.state.renderer.set_d_scale(-0.95);
                    } else {
                        this.state.renderer.set_d_scale(dScale);
                    }
                }
            }
        }
        const id = setInterval(frame, 10);
    }

    onMouseLeave() {
        const frame = () => {
            if (this.state.isRendererLoaded === true) {
                clearInterval(id);
                if (this.state.isPaused === false) {
                    this.pause();
                    this.state.isPaused = true;
                }
                this.state.isRotate = false;
                this.state.isPan = false;
                this.state.isZoom = false;
                this.state.renderer.finish_selection();
                this.state.renderer.tick();
            }
        }
        const id = setInterval(frame, 10);
    }

    onMouseDown(event) {
        const frame = () => {
            if (this.state.isRendererLoaded === true) {
                clearInterval(id);
                if (typeof event === 'object') {
                    switch (event.button) {
                    case 0:
                        if (event.ctrlKey === true && event.altKey === true && this.state.isPan === false && 
                            this.state.isZoom === false) {
                            this.state.isRotate = true;
                        }
                        if (this.state.isRotate === false) {
                            this.state.renderer.start_selection();
                        }
                        break;
                    case 1:
                        if (event.ctrlKey === true && event.altKey === true && this.state.isRotate === false && 
                            this.state.isPan === false) {
                            this.state.isZoom = true;
                        }
                        break;
                    case 2:
                        if (event.ctrlKey === true && event.altKey === true && this.state.isRotate === false && 
                            this.state.isZoom === false) {
                            this.state.isPan = true;
                        }
                        break;
                    default:
                        console.log(`Unknown button code: ${event.button}`);
                    }
                }
            }
        }
        const id = setInterval(frame, 10);
    }

    onMouseUp(event) {
        const frame = () => {
            if (this.state.isRendererLoaded === true) {
                clearInterval(id);
                if (typeof event === 'object') {
                    switch (event.button) {
                    case 0:
                        this.state.renderer.finish_selection();
                        this.state.renderer.select_objects(() => this.dropSelection());
                        if (this.state.isRotate === true) {
                            this.state.isRotate = false;
                        }
                        break;
                    case 1:
                        if (this.state.isZoom === true) {
                            this.state.isZoom = false;
                        }
                        break;
                    case 2:
                        if (this.state.isPan === true) {
                            this.state.isPan = false;
                        }
                        break;
                    default:
                        console.log(`Unknown button code: ${event.button}`);
                    }
                }
            }
        }
        const id = setInterval(frame, 10);
    }

    onMouseWheel(event) {
        const frame = () => {
            if (this.state.isRendererLoaded === true) {
                clearInterval(id);
                if (event.ctrlKey === true) {
                    event.preventDefault();
                }
                const dScale = this.state.renderer.get_d_scale() + event.deltaY / this.props.canvasHeight;
                if (1.0 + dScale > 50.0) {
                    this.state.renderer.set_d_scale(48.95);
                } else if (1.0 + dScale < 0.0) {
                    this.state.renderer.set_d_scale(-0.95);
                } else {
                    this.state.renderer.set_d_scale(dScale);
                }
            }
        }
        const id = setInterval(frame, 10);
    }

    dropSelection() {
        this.shadowRoot.querySelector(".object-info-field").innerHTML = "";
    }
}


export default FeaRenderer;
