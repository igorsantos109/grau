# Grau - Motorcycle Wheelie Game (Web Version)

A Rust/WASM rewrite of the Grau motorcycle wheelie game for the web, based on reverse-engineering the original Unity Android APK.

## Features

- **Realistic Motorcycle Physics** - Based on reverse-engineered physics parameters from the original game
- **Wheelie System** - Authentic wheelie mechanics with sustentation force, critical angle, and force reverse
- **Drift Physics** - Sideways force, reduced rear friction, drift rotation torque
- **Zerinho (Donut)** - High-steering low-speed spinning mechanic
- **Multiple Camera Modes** - Third person, first person, hood, cinematic, drone
- **Touch/Mobile Support** - Full touchscreen controls
- **WebGL2 Renderer** - Custom shaders for bike, ground, skybox
- **Web Audio API** - Engine, wind, and tire screech sounds

## Reverse-Engineered Physics Parameters

Extracted from `libil2cpp.so` and `global-metadata.dat`:

| Parameter | Value | Description |
|-----------|-------|-------------|
| `forcaSustentacaoGrau` | 150.0 | Wheelie sustentation force |
| `anguloForceReverse` | 45.0 | Force reverse angle threshold |
| `forceReverse` | 200.0 | Force reverse strength |
| `zerinhoRotationTorque` | 500.0 | Donut spin torque |
| `steerInputForZerinhoThreshold` | 0.8 | Steering threshold for zerinho |
| `zerinhoRearFriction` | 0.3 | Rear friction during zerinho |
| `zerinhoAngularDrag` | 2.0 | Angular drag during zerinho |
| `minSpeedForDrift` | 10.0 | Minimum speed for drift |
| `driftSidewaysForce` | 300.0 | Drift sideways force |
| `driftRearFriction` | 0.4 | Rear friction during drift |
| `driftFrontFriction` | 0.8 | Front friction during drift |
| `driftBrakingFactor` | 0.5 | Drift braking factor |
| `driftRotationTorque` | 200.0 | Drift rotation torque |
| `driftAngularDrag` | 1.5 | Angular drag during drift |
| `wheelieTorque` | 1000.0 | Wheelie torque |
| `wheelieDrag` | 0.1 | Wheelie drag |
| `wheelieAngularMultiplier` | 1.5 | Wheelie angular multiplier |
| `wheelieBrakeForce` | 500.0 | Wheelie brake force |
| `criticalAngle` | 75.0 | Critical wheelie angle (degrees) |
| `wheelieSpeed` | 30.0 | Minimum speed for wheelie |
| `wheelieReturnSpeed` | 5.0 | Wheelie return speed |

## Controls

| Key | Action |
|-----|--------|
| `W` / `↑` | Accelerate |
| `S` / `↓` | Brake/Reverse |
| `A` / `←` | Steer Left |
| `D` / `→` | Steer Right |
| `SPACE` / `SHIFT` | Wheelie |
| `CTRL` | Drift |
| `C` | Toggle Camera |
| `R` | Reset Bike |
| `H` | Toggle Help |

**Mobile:** Touch left side to steer, right side for throttle/brake, bottom-right for wheelie

## Building

### Prerequisites

- Rust (latest stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `wasm-pack`: `cargo install wasm-pack`

### Development Build

```bash
cd grau-web
wasm-pack build --target web --out-dir pkg --dev
```

### Release Build

```bash
cd grau-web
wasm-pack build --target web --out-dir pkg --release
```

### Running Locally

```bash
cd grau-web
# Serve the pkg directory and public folder
python3 -m http.server 8080 --directory public
# Or use any static file server
```

Then open http://localhost:8080

## Project Structure

```
grau-web/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Entry point
│   ├── game.rs          # Game loop & state
│   ├── physics.rs       # Motorcycle physics engine
│   ├── bike.rs          # Bike entity
│   ├── renderer.rs      # WebGL2 renderer
│   ├── camera.rs        # Camera system
│   ├── input.rs         # Input management
│   ├── audio.rs         # Web Audio API
│   └── ui.rs            # HTML/CSS UI overlay
├── public/
│   └── index.html       # Main HTML with styles
└── pkg/                 # Generated WASM output (after build)
```

## Architecture

### Physics Engine (`physics.rs`)
- Rigid body simulation with forces and torques
- Wheelie physics with sustentation and force reverse
- Drift physics with sideways force and friction modulation
- Zerinho (donut) physics for high-steer low-speed spinning
- Gravity, drag, and ground collision

### Renderer (`renderer.rs`)
- WebGL2 with custom vertex/fragment shaders
- Bike model (procedural cube geometry)
- Infinite ground plane with grid
- Procedural skybox with sun, clouds, stars
- Basic lighting (ambient, diffuse, specular, fresnel)

### Camera (`camera.rs`)
- 5 camera modes with smooth interpolation
- Wheelie tilt compensation
- Camera shake system

### Input (`input.rs`)
- Keyboard, mouse, and touch support
- Just-pressed/just-released tracking
- Normalized touch coordinates

### Audio (`audio.rs`)
- Web Audio API oscillators for procedural sounds
- Engine RPM-based frequency
- Wind speed-based volume/frequency
- Drift tire screech

## Credits

Original game: **Projeto Grau** by TechGameGroup
Reverse-engineered using REA (Reverse Engineering Assistant) with JADX and IL2CPP metadata analysis.

## License

This is a reverse-engineering educational project. Original game assets and IP belong to their respective owners.