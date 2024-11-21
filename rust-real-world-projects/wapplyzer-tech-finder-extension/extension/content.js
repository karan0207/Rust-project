async function detectTechnologies() {
    const wasm = await import(chrome.runtime.getURL('pkg/your_wasm_package.js'));
    const detectedTechs = wasm.detect_technologies();
    console.log("Detected Technologies:", detectedTechs);
    // Here you could add code to display this info in the popup.
  }
  
  detectTechnologies();
  