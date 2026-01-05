import { EVENT_TARGET } from "../consts/common_consts.js";
import { REL_TOL, ABS_TOL } from "../consts/common_consts.js";
import {
  TRUSS_ELEMENTS_GROUP_NUMBER,
  BEAM_ELEMENTS_GROUP_NUMBER,
  PLATE_ELEMENTS_GROUP_NUMBER,
} from "../consts/mesher_consts.js";
import {
  SOLVER_MESSAGE_EVENT_HEADER,
  CREATE_FEM_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_DIRECT_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_JACOBI_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_GPU_MESSAGE_HEADER,
  EXTRACT_GLOBAL_ANALYSIS_RESULT_MESSAGE_HEADER,
  EXTRACT_ELEMENTS_ANALYSIS_RESULT_MESSAGE_HEADER,
} from "../consts/solver_consts.js";

const worker = new Worker(
  new URL(
    "../wasm_modules_initialization/solver_initialization.js",
    import.meta.url
  )
);

export const createFEM = (jobName, mesh, solver, maxIter) => {
  const handleCreateFEM = (message) => {
    document.querySelector(EVENT_TARGET).dispatchEvent(
      new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
        bubbles: true,
        composed: true,
        detail: {
          header: message.data.header,
          job_name: message.data.job_name,
          status: message.data.status,
          message: message.data.message,
          mesh: message.data.mesh,
          solver: solver,
          max_iter: maxIter,
        },
      })
    );
    worker.removeEventListener("message", handleCreateFEM);
  };
  worker.addEventListener("message", handleCreateFEM, false);
  worker.postMessage({
    header: CREATE_FEM_MESSAGE_HEADER,
    job_name: jobName,
    mesh: mesh,
    rel_tol: REL_TOL,
    abs_tol: ABS_TOL,
    truss_elements_group_number: TRUSS_ELEMENTS_GROUP_NUMBER,
    beam_elements_group_number: BEAM_ELEMENTS_GROUP_NUMBER,
    plate_elements_group_number: PLATE_ELEMENTS_GROUP_NUMBER,
  });
};

export const performGlobalAnalysis = (jobName, solver, maxIter) => {
  const handlePerformGlobalAnalysisDirect = (message) => {
    document.querySelector(EVENT_TARGET).dispatchEvent(
      new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
        bubbles: true,
        composed: true,
        detail: {
          header: message.data.header,
          job_name: message.data.job_name,
          status: message.data.status,
          message: message.data.message,
        },
      })
    );
    worker.removeEventListener("message", handlePerformGlobalAnalysisDirect);
  };
  const handlePerformGlobalAnalysisIterative = (message) => {
    if (message.data.header !== "export_dataset") {
      document.querySelector(EVENT_TARGET).dispatchEvent(
        new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
          bubbles: true,
          composed: true,
          detail: {
            header: message.data.header,
            job_name: message.data.job_name,
            status: message.data.status,
            message: message.data.message,
            iterations: message.data.iterations,
          },
        })
      );
      worker.removeEventListener(
        "message",
        handlePerformGlobalAnalysisIterative
      );
    } else {
      const blob = new Blob([message.data.text], {
        type: "text/plain;charset=utf-8",
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = message.data.filename;
      a.style.display = "none";
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    }
  };
  if (solver === "direct") {
    worker.addEventListener(
      "message",
      handlePerformGlobalAnalysisDirect,
      false
    );
    worker.postMessage({
      header: PERFORM_GLOBAL_ANALYSIS_DIRECT_MESSAGE_HEADER,
      job_name: jobName,
    });
  }

  if (solver === "pcg_jacobi_cpu") {
    worker.addEventListener(
      "message",
      handlePerformGlobalAnalysisIterative,
      false
    );
    worker.postMessage({
      header: PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_JACOBI_MESSAGE_HEADER,
      job_name: jobName,
      max_iter: maxIter,
    });
  }

  if (solver === "pcg_block_jacobi_cpu") {
    worker.addEventListener(
      "message",
      handlePerformGlobalAnalysisIterative,
      false
    );
    worker.postMessage({
      header: PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_MESSAGE_HEADER,
      job_name: jobName,
      max_iter: maxIter,
    });
  }

  if (solver === "pcg_block_jacobi_gpu") {
    worker.addEventListener(
      "message",
      handlePerformGlobalAnalysisIterative,
      false
    );
    worker.postMessage({
      header:
        PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_GPU_MESSAGE_HEADER,
      job_name: jobName,
      max_iter: maxIter,
    });
  }
};

export const extractGlobalAnalysisResult = (jobName) => {
  const handleExtractGlobalAnalysisResult = (message) => {
    document.querySelector(EVENT_TARGET).dispatchEvent(
      new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
        bubbles: true,
        composed: true,
        detail: {
          header: message.data.header,
          job_name: message.data.job_name,
          status: message.data.status,
          message: message.data.message,
          global_analysis_result: message.data.global_analysis_result,
        },
      })
    );
    worker.removeEventListener("message", handleExtractGlobalAnalysisResult);
  };
  worker.addEventListener("message", handleExtractGlobalAnalysisResult, false);
  worker.postMessage({
    header: EXTRACT_GLOBAL_ANALYSIS_RESULT_MESSAGE_HEADER,
    job_name: jobName,
  });
};

export const extractElementsAnalysisResult = (jobName) => {
  const handleExtractElementsAnalysisResult = (message) => {
    document.querySelector(EVENT_TARGET).dispatchEvent(
      new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
        bubbles: true,
        composed: true,
        detail: {
          header: message.data.header,
          job_name: message.data.job_name,
          status: message.data.status,
          message: message.data.message,
          elements_analysis_result: message.data.elements_analysis_result,
        },
      })
    );
    worker.removeEventListener("message", handleExtractElementsAnalysisResult);
  };
  worker.addEventListener(
    "message",
    handleExtractElementsAnalysisResult,
    false
  );
  worker.postMessage({
    header: EXTRACT_ELEMENTS_ANALYSIS_RESULT_MESSAGE_HEADER,
    job_name: jobName,
  });
};
