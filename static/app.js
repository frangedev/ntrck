// Note frequencies mapping
const NOTE_FREQUENCIES = {
    'C4': 261.63, 'C#4': 277.18, 'D4': 293.66, 'D#4': 311.13,
    'E4': 329.63, 'F4': 349.23, 'F#4': 369.99, 'G4': 392.00,
    'G#4': 415.30, 'A4': 440.00, 'A#4': 466.16, 'B4': 493.88,
    'C5': 523.25, 'C#5': 554.37, 'D5': 587.33, 'D#5': 622.25,
    'E5': 659.25, 'F5': 698.46, 'F#5': 739.99, 'G5': 783.99,
    'G#5': 830.61, 'A5': 880.00, 'A#5': 932.33, 'B5': 987.77
};

// Waveform mapping
const WAVEFORM_MAP = {
    'sine': 'Sine',
    'square': 'Square', 
    'sawtooth': 'Sawtooth',
    'triangle': 'Triangle'
};

class NtrckApp {
    constructor() {
        this.engine = null;
        this.currentPattern = null;
        this.selectedWaveform = 'sine';
        this.selectedNote = 'C4';
        this.volume = 0.5;
        this.isPlaying = false;
        this.currentStep = 0;
        this.patternLength = 16;
        
        this.init();
    }

    async init() {
        try {
            // Initialize WASM module
            const wasmModule = await import('../wasm/pkg/ntrck_wasm.js');
            await wasmModule.default();
            
            this.engine = new wasmModule.NtrckEngine();
            console.log('🎛️ ntrck engine initialized');
            
            this.setupEventListeners();
            this.createPatternGrid();
            this.updateStatus('Ready');
            
        } catch (error) {
            console.error('Failed to initialize ntrck:', error);
            this.updateStatus('Failed to initialize engine');
        }
    }

    setupEventListeners() {
        // Play/Stop controls
        document.getElementById('playBtn').addEventListener('click', () => this.togglePlay());
        document.getElementById('stopBtn').addEventListener('click', () => this.stop());
        
        // BPM control
        document.getElementById('bpmInput').addEventListener('change', (e) => {
            const bpm = parseFloat(e.target.value);
            if (this.engine) {
                this.engine.set_bpm(bpm);
            }
        });

        // Pattern controls
        document.getElementById('newPatternBtn').addEventListener('click', () => this.createNewPattern());
        document.getElementById('clearPatternBtn').addEventListener('click', () => this.clearPattern());

        // Waveform selector
        document.querySelectorAll('.waveform-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                document.querySelectorAll('.waveform-btn').forEach(b => b.classList.remove('active'));
                e.target.classList.add('active');
                this.selectedWaveform = e.target.dataset.waveform;
            });
        });

        // Note selector
        document.querySelectorAll('.note-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                this.selectedNote = e.target.dataset.note;
            });
        });

        // Volume control
        document.getElementById('volumeSlider').addEventListener('input', (e) => {
            this.volume = e.target.value / 100;
            document.getElementById('volumeValue').textContent = `${e.target.value}%`;
        });
    }

    createPatternGrid() {
        const grid = document.getElementById('patternGrid');
        grid.innerHTML = '';

        for (let i = 0; i < this.patternLength; i++) {
            const row = document.createElement('div');
            row.className = 'grid-row';
            row.dataset.step = i;

            row.innerHTML = `
                <div class="step-number">${i.toString().padStart(2, '0')}</div>
                <div class="note-cell">
                    <input type="text" class="note-input" placeholder="---" data-step="${i}">
                </div>
                <div class="waveform-cell">
                    <div class="waveform-indicator" data-step="${i}"></div>
                </div>
                <div class="volume-cell">
                    <div class="volume-bar">
                        <div class="volume-fill" data-step="${i}"></div>
                    </div>
                </div>
            `;

            // Add click handlers for grid cells
            const noteInput = row.querySelector('.note-input');
            const waveformIndicator = row.querySelector('.waveform-indicator');
            const volumeFill = row.querySelector('.volume-fill');

            noteInput.addEventListener('click', () => this.setStepNote(i));
            waveformIndicator.addEventListener('click', () => this.setStepWaveform(i));
            volumeFill.parentElement.addEventListener('click', (e) => this.setStepVolume(i, e));

            grid.appendChild(row);
        }
    }

    setStepNote(step) {
        const noteInput = document.querySelector(`[data-step="${step}"]`);
        if (noteInput) {
            noteInput.value = this.selectedNote;
            this.updateStepData(step);
        }
    }

    setStepWaveform(step) {
        const indicator = document.querySelector(`.waveform-indicator[data-step="${step}"]`);
        if (indicator) {
            // Remove all waveform classes
            indicator.classList.remove('sine', 'square', 'sawtooth', 'triangle', 'active');
            // Add current waveform class
            indicator.classList.add(this.selectedWaveform, 'active');
            this.updateStepData(step);
        }
    }

    setStepVolume(step, event) {
        const volumeBar = event.currentTarget;
        const rect = volumeBar.getBoundingClientRect();
        const clickX = event.clientX - rect.left;
        const volume = Math.max(0, Math.min(1, clickX / rect.width));
        
        const volumeFill = volumeBar.querySelector('.volume-fill');
        volumeFill.style.width = `${volume * 100}%`;
        
        this.updateStepData(step);
    }

    updateStepData(step) {
        const noteInput = document.querySelector(`.note-input[data-step="${step}"]`);
        const waveformIndicator = document.querySelector(`.waveform-indicator[data-step="${step}"]`);
        const volumeFill = document.querySelector(`.volume-fill[data-step="${step}"]`);

        if (noteInput && noteInput.value && NOTE_FREQUENCIES[noteInput.value]) {
            // Create audio sample data
            const sample = {
                frequency: NOTE_FREQUENCIES[noteInput.value],
                duration: 0.25, // 16th note at 120 BPM
                amplitude: parseFloat(volumeFill.style.width) / 100 || 0.5,
                waveform: this.getWaveformFromIndicator(waveformIndicator)
            };

            // Store in pattern data
            if (!this.currentPattern) {
                this.currentPattern = this.createEmptyPattern();
            }
            this.currentPattern.steps[step] = sample;
        } else {
            // Clear step
            if (this.currentPattern) {
                this.currentPattern.steps[step] = null;
            }
        }
    }

    getWaveformFromIndicator(indicator) {
        if (indicator.classList.contains('sine')) return 'Sine';
        if (indicator.classList.contains('square')) return 'Square';
        if (indicator.classList.contains('sawtooth')) return 'Sawtooth';
        if (indicator.classList.contains('triangle')) return 'Triangle';
        return 'Sine';
    }

    createEmptyPattern() {
        return {
            id: Date.now(),
            name: 'Pattern ' + Date.now(),
            steps: new Array(this.patternLength).fill(null),
            length: this.patternLength
        };
    }

    createNewPattern() {
        this.currentPattern = this.createEmptyPattern();
        this.clearPatternGrid();
        document.getElementById('patternName').textContent = this.currentPattern.name;
        this.updateStatus('New pattern created');
    }

    clearPattern() {
        if (this.currentPattern) {
            this.currentPattern.steps = new Array(this.patternLength).fill(null);
            this.clearPatternGrid();
            this.updateStatus('Pattern cleared');
        }
    }

    clearPatternGrid() {
        document.querySelectorAll('.note-input').forEach(input => input.value = '');
        document.querySelectorAll('.waveform-indicator').forEach(indicator => {
            indicator.classList.remove('sine', 'square', 'sawtooth', 'triangle', 'active');
        });
        document.querySelectorAll('.volume-fill').forEach(fill => {
            fill.style.width = '0%';
        });
    }

    async togglePlay() {
        if (this.isPlaying) {
            this.stop();
        } else {
            this.play();
        }
    }

    async play() {
        if (!this.engine || !this.currentPattern) {
            this.updateStatus('No pattern to play');
            return;
        }

        try {
            await this.engine.start();
            this.isPlaying = true;
            document.getElementById('playBtn').textContent = '⏸️ Pause';
            this.updateStatus('Playing...');
            
            // Start step visualization
            this.startStepVisualization();
            
        } catch (error) {
            console.error('Failed to start playback:', error);
            this.updateStatus('Playback failed');
        }
    }

    stop() {
        if (this.engine) {
            this.engine.stop();
        }
        this.isPlaying = false;
        this.currentStep = 0;
        document.getElementById('playBtn').textContent = '▶️ Play';
        this.updateStatus('Stopped');
        this.clearStepVisualization();
    }

    startStepVisualization() {
        if (!this.isPlaying) return;

        // Clear previous active step
        document.querySelectorAll('.grid-row').forEach(row => {
            row.classList.remove('active');
        });

        // Highlight current step
        const currentRow = document.querySelector(`[data-step="${this.currentStep}"]`);
        if (currentRow) {
            currentRow.classList.add('active');
        }

        // Update step display
        document.getElementById('currentStep').textContent = `Step: ${this.currentStep}`;

        // Move to next step
        this.currentStep = (this.currentStep + 1) % this.patternLength;

        // Schedule next step
        const stepDuration = (60 / parseFloat(document.getElementById('bpmInput').value)) / 4; // 16th note
        setTimeout(() => {
            if (this.isPlaying) {
                this.startStepVisualization();
            }
        }, stepDuration * 1000);
    }

    clearStepVisualization() {
        document.querySelectorAll('.grid-row').forEach(row => {
            row.classList.remove('active');
        });
        document.getElementById('currentStep').textContent = 'Step: 0';
    }

    updateStatus(message) {
        document.getElementById('statusText').textContent = message;
        console.log('Status:', message);
    }

    // --- Pattern Management Stubs ---
    listPatterns() {
        // TODO: Call WASM list_patterns and update UI
        // Example: this.engine.list_patterns()
        // Show pattern list in sidebar
    }

    exportCurrentPattern() {
        // TODO: Call WASM export_pattern and trigger download
        // Example: this.engine.export_pattern(this.currentPattern.id)
    }

    importPatternFromFile(file) {
        // TODO: Read file, parse JSON, call WASM import_pattern
        // Example: this.engine.import_pattern(patternJson)
    }

    // --- Error Reporting Stub ---
    showError(message) {
        // TODO: Show error banner/popup in UI
        alert(message); // Temporary
    }
}

// Initialize the app when the page loads
document.addEventListener('DOMContentLoaded', () => {
    new NtrckApp();
}); 