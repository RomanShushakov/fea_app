import FeaApp from "./components/fea-app.js";
import FeaAppToolBar from "./components/fea-app-tool-bar/fea-app-tool-bar.js";

import FeaRenderer from "./components/fea-renderer.js";

import FeaPreprocessorMenu from "./components/fea-preprocessor/fea-preprocessor-menu.js";
import FeaPreprocessorMenuButtons from "./components/fea-preprocessor/fea-preprocessor-menu-buttons.js";

import AddButton from "./components/common/crud-buttons/add-button.js";
import UpdateButton from "./components/common/crud-buttons/update-button.js";
import DeleteButton from "./components/common/crud-buttons/delete-button.js";

import AssignButton from "./components/common/assign-button.js";
import BeamSectionOrientationButton from "./components/common/beam-section-orientation-button.js";

import FeaGeometryMenu from "./components/fea-preprocessor/fea-geometry/fea-geometry-menu.js";
import FeaGeometryMenuButtons from "./components/fea-preprocessor/fea-geometry/fea-geometry-menu-buttons.js";
import FeaGeometryPointMenu from "./components/fea-preprocessor/fea-geometry/point/fea-geometry-point-menu.js";
import FeaGeometryPointMenuButtons 
    from "./components/fea-preprocessor/fea-geometry/point/fea-geometry-point-menu-buttons.js";
import FeaGeometryLineMenu from "./components/fea-preprocessor/fea-geometry/line/fea-geometry-line-menu.js";
import FeaGeometryLineMenuButtons 
    from "./components/fea-preprocessor/fea-geometry/line/fea-geometry-line-menu-buttons.js";
import FeaGeometrySurfaceMenu from "./components/fea-preprocessor/fea-geometry/surface/fea-geometry-surface-menu.js";
import FeaGeometrySurfaceMenuButtons 
    from "./components/fea-preprocessor/fea-geometry/surface/fea-geometry-surface-menu-buttons.js";
import FeaGeometryAddPointMenu 
    from "./components/fea-preprocessor/fea-geometry/point/fea-geometry-add-point-menu.js";
import FeaGeometryUpdatePointMenu 
    from "./components/fea-preprocessor/fea-geometry/point/fea-geometry-update-point-menu.js";
import FeaGeometryDeletePointMenu 
    from "./components/fea-preprocessor/fea-geometry/point/fea-geometry-delete-point-menu.js";
import FeaGeometryAddLineMenu from "./components/fea-preprocessor/fea-geometry/line/fea-geometry-add-line-menu.js";
import FeaGeometryUpdateLineMenu 
    from "./components/fea-preprocessor/fea-geometry/line/fea-geometry-update-line-menu.js";
import FeaGeometryDeleteLineMenu 
    from "./components/fea-preprocessor/fea-geometry/line/fea-geometry-delete-line-menu.js";
import FeaGeometryAddSurfaceMenu 
    from "./components/fea-preprocessor/fea-geometry/surface/fea-geometry-add-surface-menu.js";
import FeaGeometryUpdateSurfaceMenu 
    from "./components/fea-preprocessor/fea-geometry/surface/fea-geometry-update-surface-menu.js";
import FeaGeometryDeleteSurfaceMenu 
    from "./components/fea-preprocessor/fea-geometry/surface/fea-geometry-delete-surface-menu.js";

import FeaMaterialMenu from "./components/fea-preprocessor/fea-material/fea-material-menu.js";
import FeaMaterialMenuButtons from "./components/fea-preprocessor/fea-material/fea-material-menu-buttons.js";
import FeaMaterialAddMaterialMenu 
    from "./components/fea-preprocessor/fea-material/fea-material-add-material-menu.js";
import FeaMaterialUpdateMaterialMenu 
    from "./components/fea-preprocessor/fea-material/fea-material-update-material-menu.js";
import FeaMaterialDeleteMaterialMenu 
    from "./components/fea-preprocessor/fea-material/fea-material-delete-material-menu.js";

import FeaSectionMenu from "./components/fea-preprocessor/fea-section/fea-section-menu.js";
import FeaSectionMenuButtons from "./components/fea-preprocessor/fea-section/fea-section-menu-buttons.js";

import FeaSectionTrussMenu from "./components/fea-preprocessor/fea-section/truss/fea-section-truss-menu.js";
import FeaSectionTrussMenuButtons 
    from "./components/fea-preprocessor/fea-section/truss/fea-section-truss-menu-buttons.js";
import FeaSectionAddTrussMenu from "./components/fea-preprocessor/fea-section/truss/fea-section-add-truss-menu.js";
import FeaSectionUpdateTrussMenu 
    from "./components/fea-preprocessor/fea-section/truss/fea-section-update-truss-menu.js";
import FeaSectionDeleteTrussMenu 
    from "./components/fea-preprocessor/fea-section/truss/fea-section-delete-truss-menu.js";

import FeaSectionBeamMenu from "./components/fea-preprocessor/fea-section/beam/fea-section-beam-menu.js";
import FeaSectionBeamMenuButtons 
    from "./components/fea-preprocessor/fea-section/beam/fea-section-beam-menu-buttons.js";
import FeaSectionAddBeamMenu from "./components/fea-preprocessor/fea-section/beam/fea-section-add-beam-menu.js";
import FeaSectionUpdateBeamMenu 
    from "./components/fea-preprocessor/fea-section/beam/fea-section-update-beam-menu.js";
import FeaSectionDeleteBeamMenu 
    from "./components/fea-preprocessor/fea-section/beam/fea-section-delete-beam-menu.js";

import FeaSectionPlateMenu from "./components/fea-preprocessor/fea-section/plate/fea-section-plate-menu.js";
import FeaSectionPlateMenuButtons 
    from "./components/fea-preprocessor/fea-section/plate/fea-section-plate-menu-buttons.js";
import FeaSectionAddPlateMenu from "./components/fea-preprocessor/fea-section/plate/fea-section-add-plate-menu.js";
import FeaSectionUpdatePlateMenu 
    from "./components/fea-preprocessor/fea-section/plate/fea-section-update-plate-menu.js";
import FeaSectionDeletePlateMenu 
    from "./components/fea-preprocessor/fea-section/plate/fea-section-delete-plate-menu.js";

import FeaPropertiesMenu from "./components/fea-preprocessor/fea-properties/fea-properties-menu.js";
import FeaPropertiesMenuButtons from "./components/fea-preprocessor/fea-properties/fea-properties-menu-buttons.js";
import FeaPropertiesAddPropertiesMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-add-properties-menu.js";
import FeaPropertiesUpdatePropertiesMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-update-properties-menu.js";
import FeaPropertiesDeletePropertiesMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-delete-properties-menu.js";
import FeaPropertiesAssignPropertiesMenuButtons 
    from "./components/fea-preprocessor/fea-properties/fea-properties-assign-properties-menu-buttons.js";
import FeaPropertiesAssignPropertiesMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-assign-properties-menu.js";
import FeaPropertiesAssignPropertiesToLinesMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-assign-properties-to-lines-menu.js";
import FeaPropertiesAssignPropertiesToSurfacesMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-assign-properties-to-surfaces-menu.js";
import FeaPropertiesBeamSectionOrientationMenu 
    from "./components/fea-preprocessor/fea-properties/fea-properties-beam-section-orientation-menu.js";

import FeaLoadMenu from "./components/fea-preprocessor/fea-load/fea-load-menu.js";
import FeaLoadConcentratedLoadMenu 
    from "./components/fea-preprocessor/fea-load/concentrated/fea-load-concentrated-load-menu.js";
import FeaLoadConcentratedLoadMenuButtons 
    from "./components/fea-preprocessor/fea-load/concentrated/fea-load-concentrated-load-menu-buttons.js";
import FeaLoadAddConcentratedLoadMenu 
    from "./components/fea-preprocessor/fea-load/concentrated/fea-load-add-concentrated-load-menu.js";
import FeaLoadUpdateConcentratedLoadMenu 
    from "./components/fea-preprocessor/fea-load/concentrated/fea-load-update-concentrated-load-menu.js";
import FeaLoadDeleteConcentratedLoadMenu 
    from "./components/fea-preprocessor/fea-load/concentrated/fea-load-delete-concentrated-load-menu.js";
import FeaLoadUniformlyDistributedLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/fea-load-uniformly-distributed-load-menu.js";
import FeaLoadUniformlyDistributedLoadMenuButtons 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/fea-load-uniformly-distributed-load-menu-buttons.js";
import FeaLoadUniformlyDistributedLineLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/line/fea-load-uniformly-distributed-line-load-menu.js";
import FeaLoadUniformlyDistributedLineLoadMenuButtons 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/line/fea-load-uniformly-distributed-line-load-menu-buttons.js";
import FeaLoadAddUniformlyDistributedLineLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/line/fea-load-add-uniformly-distributed-line-load-menu.js";
import FeaLoadUpdateUniformlyDistributedLineLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/line/fea-load-update-uniformly-distributed-line-load-menu.js";
import FeaLoadDeleteUniformlyDistributedLineLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/line/fea-load-delete-uniformly-distributed-line-load-menu.js";
import FeaLoadUniformlyDistributedSurfaceLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/surface/fea-load-uniformly-distributed-surface-load-menu.js";
import FeaLoadUniformlyDistributedSurfaceLoadMenuButtons 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/surface/fea-load-uniformly-distributed-surface-load-menu-buttons.js";
import FeaLoadAddUniformlyDistributedSurfaceLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/surface/fea-load-add-uniformly-distributed-surface-load-menu.js";
import FeaLoadUpdateUniformlyDistributedSurfaceLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/surface/fea-load-update-uniformly-distributed-surface-load-menu.js";
import FeaLoadDeleteUniformlyDistributedSurfaceLoadMenu 
    from "./components/fea-preprocessor/fea-load/uniformly_distributed/surface/fea-load-delete-uniformly-distributed-surface-load-menu.js";

import FeaBoundaryConditionMenu from "./components/fea-preprocessor/fea-boundary-condition/fea-boundary-condition-menu.js";
import FeaBoundaryConditionMenuButtons 
    from "./components/fea-preprocessor/fea-boundary-condition/fea-boundary-condition-menu-buttons.js";
import FeaBoundaryConditionAddBoundaryConditionMenu from 
    "./components/fea-preprocessor/fea-boundary-condition/fea-boundary-condition-add-boundary-condition-menu.js";
import FeaBoundaryConditionUpdateBoundaryConditionMenu from 
    "./components/fea-preprocessor/fea-boundary-condition/fea-boundary-condition-update-boundary-condition-menu.js";
import FeaBoundaryConditionDeleteBoundaryConditionMenu from 
    "./components/fea-preprocessor/fea-boundary-condition/fea-boundary-condition-delete-boundary-condition-menu.js";

import FeaMeshMenu from "./components/fea-preprocessor/fea-mesh/fea-mesh-menu.js";
import FeaMeshMenuButtons from "./components/fea-preprocessor/fea-mesh/fea-mesh-menu-buttons.js";
import FeaMeshGlobalMeshSeedMenu from "./components/fea-preprocessor/fea-mesh/global_mesh_seed/fea-mesh-global-mesh-seed-menu.js";
import FeaMeshLocalMeshSeedMenu from "./components/fea-preprocessor/fea-mesh/local_mesh_seed/fea-mesh-local-mesh-seed-menu.js";
import FeaMeshLocalMeshSeedMenuButtons 
    from "./components/fea-preprocessor/fea-mesh/local_mesh_seed/fea-mesh-local-mesh-seed-menu-buttons.js";
import FeaMeshLinesMeshSeedMenu 
    from "./components/fea-preprocessor/fea-mesh/local_mesh_seed/lines/fea-mesh-lines-mesh-seed-menu.js";
import FeaMeshSurfacesMeshSeedMenu 
    from "./components/fea-preprocessor/fea-mesh/local_mesh_seed/surfaces/fea-mesh-surfaces-mesh-seed-menu.js";

import FeaAnalysisMenu from "./components/fea-preprocessor/fea-analysis/fea-analysis-menu.js";

import FeaPostprocessorMenu from "./components/fea-postprocessor/fea-postprocessor-menu.js";
import FeaPostprocessorMenuButtons from "./components/fea-postprocessor/fea-postprocessor-menu-buttons.js";

import FeaContoursMenu from "./components/fea-postprocessor/fea-contours/fea-contours-menu.js";
import FeaContoursDisplacementMenu from "./components/fea-postprocessor/fea-contours/fea-contours-displacement-menu.js";

import FeaSymbolsMenu from "./components/fea-postprocessor/fea-symbols/fea-symbols-menu.js";
import FeaSymbolsGlobalLoadsMenu 
    from "./components/fea-postprocessor/fea-symbols/fea-symbols-global-loads-menu.js";
import FeaSymbolsElementsLoadsMenu 
    from "./components/fea-postprocessor/fea-symbols/fea-symbols-elements-loads-menu.js";

import FeaAppLoadButton from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-load-button.js";
import FeaAppResetButton from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-reset-button.js";
import FeaAppUndoButton from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-undo-button.js";
import FeaAppRedoButton from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-redo-button.js";
import FeaAppShowHidePointButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-point-button.js";
import FeaAppShowHideLineButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-line-button.js";
import FeaAppShowHideSurfaceButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-surface-button.js";
import FeaAppShowHideSurfaceEdges13Button 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-surface-edges-1-3-button.js";
import FeaAppShowHideSurfaceEdges24Button 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-surface-edges-2-4-button.js";
import FeaAppShowHideSurfaceNormalButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-surface-normal-button.js";
import FeaAppShowHideBeamSectionOrientationButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-beam-section-orientation-button.js";
import FeaAppShowHideLoadButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-load-button.js";
import FeaAppShowHideBoundaryConditionButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-boundary-condition-button.js";
import FeaAppShowHideMeshSeedButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-mesh-seed-button.js";

import FeaAppShowHideNodeButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-node-button.js";
import FeaAppShowHideTrussElementButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-truss-element-button.js";
import FeaAppShowHideBeamElementButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-beam-element-button.js";
import FeaAppShowHidePlateElementButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-plate-element-button.js";
import FeaAppShowHideLocalAxesButton 
    from "./components/fea-app-tool-bar/context-dependent-buttons/fea-app-show-hide-local-axes-button.js";

customElements.define("fea-app", FeaApp);
customElements.define("fea-app-tool-bar", FeaAppToolBar);

customElements.define("fea-renderer", FeaRenderer);

customElements.define("fea-preprocessor-menu", FeaPreprocessorMenu);
customElements.define("fea-preprocessor-menu-buttons", FeaPreprocessorMenuButtons);

customElements.define("add-button", AddButton);
customElements.define("update-button", UpdateButton);
customElements.define("delete-button", DeleteButton);

customElements.define("assign-button", AssignButton);
customElements.define("beam-section-orientation-button", BeamSectionOrientationButton);

customElements.define("fea-geometry-menu", FeaGeometryMenu);
customElements.define("fea-geometry-menu-buttons", FeaGeometryMenuButtons);
customElements.define("fea-geometry-point-menu", FeaGeometryPointMenu);
customElements.define("fea-geometry-point-menu-buttons", FeaGeometryPointMenuButtons);
customElements.define("fea-geometry-line-menu", FeaGeometryLineMenu);
customElements.define("fea-geometry-line-menu-buttons", FeaGeometryLineMenuButtons);
customElements.define("fea-geometry-surface-menu", FeaGeometrySurfaceMenu);
customElements.define("fea-geometry-surface-menu-buttons", FeaGeometrySurfaceMenuButtons);
customElements.define("fea-geometry-add-point-menu", FeaGeometryAddPointMenu);
customElements.define("fea-geometry-update-point-menu", FeaGeometryUpdatePointMenu);
customElements.define("fea-geometry-delete-point-menu", FeaGeometryDeletePointMenu);
customElements.define("fea-geometry-add-line-menu", FeaGeometryAddLineMenu);
customElements.define("fea-geometry-update-line-menu", FeaGeometryUpdateLineMenu);
customElements.define("fea-geometry-delete-line-menu", FeaGeometryDeleteLineMenu);
customElements.define("fea-geometry-add-surface-menu", FeaGeometryAddSurfaceMenu);
customElements.define("fea-geometry-update-surface-menu", FeaGeometryUpdateSurfaceMenu);
customElements.define("fea-geometry-delete-surface-menu", FeaGeometryDeleteSurfaceMenu);

customElements.define("fea-material-menu", FeaMaterialMenu);
customElements.define("fea-material-menu-buttons", FeaMaterialMenuButtons);
customElements.define("fea-material-add-material-menu", FeaMaterialAddMaterialMenu);
customElements.define("fea-material-update-material-menu", FeaMaterialUpdateMaterialMenu);
customElements.define("fea-material-delete-material-menu", FeaMaterialDeleteMaterialMenu);

customElements.define("fea-section-menu", FeaSectionMenu);
customElements.define("fea-section-menu-buttons", FeaSectionMenuButtons);

customElements.define("fea-section-truss-menu", FeaSectionTrussMenu);
customElements.define("fea-section-truss-menu-buttons", FeaSectionTrussMenuButtons);
customElements.define("fea-section-add-truss-menu", FeaSectionAddTrussMenu);
customElements.define("fea-section-update-truss-menu", FeaSectionUpdateTrussMenu);
customElements.define("fea-section-delete-truss-menu", FeaSectionDeleteTrussMenu);

customElements.define("fea-section-beam-menu", FeaSectionBeamMenu);
customElements.define("fea-section-beam-menu-buttons", FeaSectionBeamMenuButtons);
customElements.define("fea-section-add-beam-menu", FeaSectionAddBeamMenu);
customElements.define("fea-section-update-beam-menu", FeaSectionUpdateBeamMenu);
customElements.define("fea-section-delete-beam-menu", FeaSectionDeleteBeamMenu);

customElements.define("fea-section-plate-menu", FeaSectionPlateMenu);
customElements.define("fea-section-plate-menu-buttons", FeaSectionPlateMenuButtons);
customElements.define("fea-section-add-plate-menu", FeaSectionAddPlateMenu);
customElements.define("fea-section-update-plate-menu", FeaSectionUpdatePlateMenu);
customElements.define("fea-section-delete-plate-menu", FeaSectionDeletePlateMenu);

customElements.define("fea-properties-menu", FeaPropertiesMenu);
customElements.define("fea-properties-menu-buttons", FeaPropertiesMenuButtons);
customElements.define("fea-properties-add-properties-menu", FeaPropertiesAddPropertiesMenu);
customElements.define("fea-properties-update-properties-menu", FeaPropertiesUpdatePropertiesMenu);
customElements.define("fea-properties-delete-properties-menu", FeaPropertiesDeletePropertiesMenu);
customElements.define("fea-properties-assign-properties-menu-buttons", FeaPropertiesAssignPropertiesMenuButtons);
customElements.define("fea-properties-assign-properties-menu", FeaPropertiesAssignPropertiesMenu);
customElements.define("fea-properties-assign-properties-to-lines-menu", FeaPropertiesAssignPropertiesToLinesMenu);
customElements.define("fea-properties-assign-properties-to-surfaces-menu", FeaPropertiesAssignPropertiesToSurfacesMenu);
customElements.define("fea-properties-beam-section-orientation-menu", FeaPropertiesBeamSectionOrientationMenu);

customElements.define("fea-load-menu", FeaLoadMenu);
customElements.define("fea-load-concentrated-load-menu", FeaLoadConcentratedLoadMenu);
customElements.define("fea-load-concentrated-load-menu-buttons", FeaLoadConcentratedLoadMenuButtons);
customElements.define("fea-load-add-concentrated-load-menu", FeaLoadAddConcentratedLoadMenu);
customElements.define("fea-load-update-concentrated-load-menu", FeaLoadUpdateConcentratedLoadMenu);
customElements.define("fea-load-delete-concentrated-load-menu", FeaLoadDeleteConcentratedLoadMenu);
customElements.define("fea-load-uniformly-distributed-load-menu", FeaLoadUniformlyDistributedLoadMenu);
customElements.define("fea-load-uniformly-distributed-load-menu-buttons", FeaLoadUniformlyDistributedLoadMenuButtons);
customElements.define("fea-load-uniformly-distributed-line-load-menu", FeaLoadUniformlyDistributedLineLoadMenu);
customElements.define("fea-load-uniformly-distributed-line-load-menu-buttons", FeaLoadUniformlyDistributedLineLoadMenuButtons);
customElements.define("fea-load-add-uniformly-distributed-line-load-menu", FeaLoadAddUniformlyDistributedLineLoadMenu);
customElements.define("fea-load-update-uniformly-distributed-line-load-menu", FeaLoadUpdateUniformlyDistributedLineLoadMenu);
customElements.define("fea-load-delete-uniformly-distributed-line-load-menu", FeaLoadDeleteUniformlyDistributedLineLoadMenu);
customElements.define("fea-load-uniformly-distributed-surface-load-menu", FeaLoadUniformlyDistributedSurfaceLoadMenu);
customElements.define("fea-load-uniformly-distributed-surface-load-menu-buttons", FeaLoadUniformlyDistributedSurfaceLoadMenuButtons);
customElements.define("fea-load-add-uniformly-distributed-surface-load-menu", FeaLoadAddUniformlyDistributedSurfaceLoadMenu);
customElements.define("fea-load-update-uniformly-distributed-surface-load-menu", FeaLoadUpdateUniformlyDistributedSurfaceLoadMenu);
customElements.define("fea-load-delete-uniformly-distributed-surface-load-menu", FeaLoadDeleteUniformlyDistributedSurfaceLoadMenu);

customElements.define("fea-boundary-condition-menu", FeaBoundaryConditionMenu);
customElements.define("fea-boundary-condition-menu-buttons", FeaBoundaryConditionMenuButtons);
customElements.define("fea-boundary-condition-add-boundary-condition-menu", FeaBoundaryConditionAddBoundaryConditionMenu);
customElements.define("fea-boundary-condition-update-boundary-condition-menu", FeaBoundaryConditionUpdateBoundaryConditionMenu);
customElements.define("fea-boundary-condition-delete-boundary-condition-menu", FeaBoundaryConditionDeleteBoundaryConditionMenu);

customElements.define("fea-mesh-menu", FeaMeshMenu);
customElements.define("fea-mesh-menu-buttons", FeaMeshMenuButtons);
customElements.define("fea-mesh-global-mesh-seed-menu", FeaMeshGlobalMeshSeedMenu);
customElements.define("fea-mesh-local-mesh-seed-menu", FeaMeshLocalMeshSeedMenu);
customElements.define("fea-mesh-local-mesh-seed-menu-buttons", FeaMeshLocalMeshSeedMenuButtons);
customElements.define("fea-mesh-lines-mesh-seed-menu", FeaMeshLinesMeshSeedMenu);
customElements.define("fea-mesh-surfaces-mesh-seed-menu", FeaMeshSurfacesMeshSeedMenu);

customElements.define("fea-analysis-menu", FeaAnalysisMenu);

customElements.define("fea-postprocessor-menu", FeaPostprocessorMenu);
customElements.define("fea-postprocessor-menu-buttons", FeaPostprocessorMenuButtons);

customElements.define("fea-contours-menu", FeaContoursMenu);
customElements.define("fea-contours-displacement-menu", FeaContoursDisplacementMenu);

customElements.define("fea-symbols-menu", FeaSymbolsMenu);
customElements.define("fea-symbols-global-loads-menu", FeaSymbolsGlobalLoadsMenu);
customElements.define("fea-symbols-elements-loads-menu", FeaSymbolsElementsLoadsMenu);

customElements.define("fea-app-load-button", FeaAppLoadButton);
customElements.define("fea-app-reset-button", FeaAppResetButton);
customElements.define("fea-app-undo-button", FeaAppUndoButton);
customElements.define("fea-app-redo-button", FeaAppRedoButton);
customElements.define("fea-app-show-hide-point-button", FeaAppShowHidePointButton);
customElements.define("fea-app-show-hide-line-button", FeaAppShowHideLineButton);
customElements.define("fea-app-show-hide-surface-edges-1-3-button", FeaAppShowHideSurfaceEdges13Button);
customElements.define("fea-app-show-hide-surface-edges-2-4-button", FeaAppShowHideSurfaceEdges24Button);
customElements.define("fea-app-show-hide-surface-button", FeaAppShowHideSurfaceButton);
customElements.define("fea-app-show-hide-surface-normal-button", FeaAppShowHideSurfaceNormalButton);
customElements.define("fea-app-show-hide-beam-section-orientation-button", FeaAppShowHideBeamSectionOrientationButton);
customElements.define("fea-app-show-hide-load-button", FeaAppShowHideLoadButton);
customElements.define("fea-app-show-hide-boundary-condition-button", FeaAppShowHideBoundaryConditionButton);
customElements.define("fea-app-show-hide-mesh-seed-button", FeaAppShowHideMeshSeedButton);

customElements.define("fea-app-show-hide-node-button", FeaAppShowHideNodeButton);
customElements.define("fea-app-show-hide-truss-element-button", FeaAppShowHideTrussElementButton);
customElements.define("fea-app-show-hide-beam-element-button", FeaAppShowHideBeamElementButton);
customElements.define("fea-app-show-hide-plate-element-button", FeaAppShowHidePlateElementButton);
customElements.define("fea-app-show-hide-local-axes-button", FeaAppShowHideLocalAxesButton);
