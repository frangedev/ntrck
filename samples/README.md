# 🎵 Sample Pack

This directory contains sample audio files and presets for ntrck.

## Structure

- `presets/` - Pattern presets and configurations
- `samples/` - Audio sample files (WAV, MP3, etc.)
- `instruments/` - Instrument definitions and mappings

## Adding Samples

1. Place your audio files in the `samples/` directory
2. Supported formats: WAV, MP3, FLAC, OGG
3. Use descriptive filenames (e.g., `kick_drum_808.wav`)
4. Keep file sizes reasonable for web loading

## Presets

Presets are JSON files that define:
- Pattern sequences
- Instrument mappings
- Effect chains
- Tempo and timing

Example preset structure:
```json
{
  "name": "Techno Beat",
  "bpm": 128,
  "patterns": [
    {
      "name": "Kick Pattern",
      "steps": [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]
    }
  ]
}
```

## Default Samples

The following samples are included by default:
- Basic drum kit (kick, snare, hi-hat)
- Simple waveforms (sine, square, saw, triangle)
- Bass and lead presets 