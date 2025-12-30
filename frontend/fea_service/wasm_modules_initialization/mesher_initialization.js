import init, { Mesher } from "../wasm/mesher/mesher.js";
import { MAX_POINT_NUMBER, MAX_LINE_NUMBER, MAX_SURFACE_NUMBER } from "../consts/common_consts.js";
import 
{ 
    TRUSS_ELEMENTS_GROUP_NUMBER, BEAM_ELEMENTS_GROUP_NUMBER, PLATE_ELEMENTS_GROUP_NUMBER, 
} from "../consts/mesher_consts.js";
import { ABS_TOL, REL_TOL } from "../consts/common_consts.js";


export async function initializeMesher() {
    await init();
    const mesher = Mesher.create(
        ABS_TOL,
        REL_TOL,
        MAX_POINT_NUMBER,
        MAX_LINE_NUMBER,
        MAX_SURFACE_NUMBER,
        TRUSS_ELEMENTS_GROUP_NUMBER, 
        BEAM_ELEMENTS_GROUP_NUMBER,
        PLATE_ELEMENTS_GROUP_NUMBER,
    );
    return mesher;    
}
