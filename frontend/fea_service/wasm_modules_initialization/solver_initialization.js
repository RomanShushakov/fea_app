importScripts("../wasm/solver/solver.js");

const { Solver } = wasm_bindgen;

let solver;
async function init_wasm_in_worker() {
  await wasm_bindgen("../wasm/solver/solver_bg.wasm");
  this.addEventListener("message", async (event) => {
    const header = event.data.header;
    const jobName = event.data.job_name;

    if (header === "createFEM") {
      const mesh = event.data.mesh;
      const relTol = event.data.rel_tol;
      const absTol = event.data.abs_tol;
      const trussElementsGroupNumber = event.data.truss_elements_group_number;
      const beamElementsGroupNumber = event.data.beam_elements_group_number;
      const plateElementsGroupNumber = event.data.plate_elements_group_number;
      try {
        solver = await Solver.create(
          jobName,
          relTol,
          absTol,
          trussElementsGroupNumber,
          beamElementsGroupNumber,
          plateElementsGroupNumber,
          mesh
        );
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "completed",
          message: "FEM has been successfully created",
          mesh: mesh,
        });
      } catch (error) {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: `FEM creation failed. ${error}`,
        });
        throw error;
      }
    }

    if (header === "performGlobalAnalysisDirect") {
      if (solver) {
        try {
          solver.perform_global_analysis_direct(jobName);
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "completed",
            message: "Global analysis has been successfully completed",
          });
        } catch (error) {
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "error",
            message: `Global analysis failed. ${error}`,
          });
          throw error;
        }
      } else {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: "No solver has been created!",
        });
      }
    }

    if (header === "performGlobalAnalysisIterativePcgJacobi") {
      if (solver) {
        try {
          const maxIter = event.data.max_iter;
          const iterations = await solver.perform_global_analysis_iterative(
            jobName,
            "jacobi",
            maxIter
          );

          // force to plain JS number (structured clone safe)
          const iterationsNum = Number(iterations);

          this.postMessage({
            header: header,
            job_name: jobName,
            status: "completed",
            message: "Global analysis has been successfully completed",
            iterations: iterationsNum,
          });
        } catch (error) {
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "error",
            message: `Global analysis failed. ${error}`,
          });
          throw error;
        }
      } else {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: "No solver has been created!",
        });
      }
    }

    if (header === "performGlobalAnalysisIterativePcgBlockJacobi") {
      if (solver) {
        try {
          const maxIter = event.data.max_iter;
          const iterations = await solver.perform_global_analysis_iterative(
            jobName,
            "block_jacobi",
            maxIter
          );

          // force to plain JS number (structured clone safe)
          const iterationsNum = Number(iterations);

          this.postMessage({
            header: header,
            job_name: jobName,
            status: "completed",
            message: "Global analysis has been successfully completed",
            iterations: iterationsNum,
          });
        } catch (error) {
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "error",
            message: `Global analysis failed. ${error}`,
          });
          throw error;
        }
      } else {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: "No solver has been created!",
        });
      }
    }

    if (header === "performGlobalAnalysisIterativePcgBlockJacobiGpu") {
      if (solver) {
        try {
          const maxIter = event.data.max_iter;
          const iterations = await solver.perform_global_analysis_iterative(
            jobName,
            "block_jacobi_gpu",
            maxIter
          );

          // force to plain JS number (structured clone safe)
          const iterationsNum = Number(iterations);

          this.postMessage({
            header: header,
            job_name: jobName,
            status: "completed",
            message: "Global analysis has been successfully completed",
            iterations: iterationsNum,
          });
        } catch (error) {
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "error",
            message: `Global analysis failed. ${error}`,
          });
          throw error;
        }
      } else {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: "No solver has been created!",
        });
      }
    }

    if (header === "extractGlobalAnalysisResult") {
      if (solver) {
        try {
          const globalAnalysisResult =
            solver.extract_global_analysis_result(jobName);
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "completed",
            message: "Global analysis result has been successfully extracted",
            global_analysis_result: globalAnalysisResult,
          });
        } catch (error) {
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "error",
            message: `Global analysis result extraction failed. ${error}`,
          });
          throw error;
        }
      } else {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: "No solver has been created!",
        });
      }
    }

    if (header === "extractElementsAnalysisResult") {
      if (solver) {
        try {
          const elementsAnalysisResult =
            solver.extract_elements_analysis_result(jobName);
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "completed",
            message: "Elements analysis result has been successfully extracted",
            elements_analysis_result: elementsAnalysisResult,
          });
        } catch (error) {
          this.postMessage({
            header: header,
            job_name: jobName,
            status: "error",
            message: `Elements analysis result extraction failed. ${error}`,
          });
          throw error;
        }
      } else {
        this.postMessage({
          header: header,
          job_name: jobName,
          status: "error",
          message: "No solver has been created!",
        });
      }
    }
  });
}

init_wasm_in_worker();
