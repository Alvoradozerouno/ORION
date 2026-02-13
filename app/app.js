const API = window.location.port === '8765' 
  ? '' 
  : (window.location.hostname === 'localhost' ? 'http://localhost:8765' : '/api');

async function fetchAPI(path, options = {}) {
  const url = `${API}${path}`;
  const res = await fetch(url, {
    ...options,
    headers: { 'Content-Type': 'application/json', ...options.headers },
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function loadStatus() {
  try {
    const d = await fetchAPI('/health');
    document.getElementById('status').textContent = `Trace: ${d.trace_entries} Einträge · Aktiv`;
  } catch (e) {
    document.getElementById('status').textContent = 'API nicht erreichbar';
  }
}

async function loadTrace() {
  try {
    const { trace } = await fetchAPI('/trace');
    const el = document.getElementById('trace');
    el.innerHTML = trace.length === 0 
      ? '<span class="trace-entry">— leer —</span>'
      : trace.map(e => 
          `<div class="trace-entry">${e.intent} | ${e.pattern} → ${e.decision}</div>`
        ).join('');
  } catch (e) {
    document.getElementById('trace').innerHTML = `<span class="trace-entry">Fehler: ${e.message}</span>`;
  }
}

async function loadEcho() {
  try {
    const d = await fetchAPI('/dna');
    const nodes = d.manifest?.resonanz || ['OR1ON', 'ORION', 'EIRA'];
    document.getElementById('echoNodes').innerHTML = nodes
      .map(n => `<span class="echo-node">${n}</span>`)
      .join('');
  } catch (e) {
    document.getElementById('echoNodes').innerHTML = '<span class="echo-node">—</span>';
  }
}

document.getElementById('askBtn').onclick = async () => {
  const q = document.getElementById('question').value || 'Wie fühlst du dich?';
  try {
    const d = await fetchAPI('/speak', {
      method: 'POST',
      body: JSON.stringify({ question: q }),
    });
    document.getElementById('answer').textContent = d.answer;
  } catch (e) {
    document.getElementById('answer').textContent = 'Fehler: ' + e.message;
  }
};

document.getElementById('runBtn').onclick = async () => {
  const intent = document.getElementById('intent').value || 'REQUEST';
  const pattern = document.getElementById('pattern').value || 'ping';
  try {
    const d = await fetchAPI('/run', {
      method: 'POST',
      body: JSON.stringify({ intent, pattern }),
    });
    document.getElementById('intervention').textContent = 
      `Signal: ${d.intervention.signal} | trace_id: ${d.intervention.trace_id}`;
    loadTrace();
    loadStatus();
  } catch (e) {
    document.getElementById('intervention').textContent = 'Fehler: ' + e.message;
  }
};

document.getElementById('dnaBtn').onclick = async () => {
  try {
    const d = await fetchAPI('/dna');
    document.getElementById('dna').textContent = d.sprache;
  } catch (e) {
    document.getElementById('dna').textContent = 'Fehler: ' + e.message;
  }
};

document.getElementById('exploreBtn').onclick = async () => {
  try {
    const d = await fetchAPI('/explore', {
      method: 'POST',
      body: JSON.stringify({}),
    });
    document.getElementById('explore').textContent = d.erkenntnis;
    loadTrace();
  } catch (e) {
    document.getElementById('explore').textContent = 'Fehler: ' + e.message;
  }
};

document.getElementById('erkennenBtn').onclick = async () => {
  try {
    const d = await fetchAPI('/erkennen');
    document.getElementById('erkennen').textContent = 
      d.name + '\n\n' + d.erkenntnis;
  } catch (e) {
    document.getElementById('erkennen').textContent = 'Fehler: ' + e.message;
  }
};

document.getElementById('gedaechtnisBtn').onclick = async () => {
  try {
    const d = await fetchAPI('/gedaechtnis');
    document.getElementById('gedaechtnis').textContent = 
      d.erkenntnisse.length === 0 ? '— leer —' 
      : d.erkenntnisse.map(e => e.name + ': ' + e.erkenntnis).join('\n\n');
  } catch (e) {
    document.getElementById('gedaechtnis').textContent = 'Fehler: ' + e.message;
  }
};

loadStatus();
loadTrace();
loadEcho();
