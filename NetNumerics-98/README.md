# NetNumerics 98

A retro network-security analytics workstation that turns numerical methods into transparent, reproducible defensive telemetry analysis. Built for a Numerical Methods course; no external services or dependencies in offline mode.

## Try it

Open `index.html` in your browser (extract the entire folder first). The default scenario is a labeled synthetic sustained surge. Use **Data & settings** to switch scenarios, change the training window, threshold or regression degree, or import an authorized CSV. **Root solver** shows each iteration; **Numerical methods** shows the actual computations. **Export report** downloads JSON.

For local HTTP mode: `python -m http.server 8000`, then open `http://localhost:8000`. The app loads the optional Rust/WASM module only if present and served over HTTP.

## Build the Rust module

The browser includes a complete JavaScript fallback; Rust provides the quadrature and Richardson kernels. With Rust installed, run:

```sh
rustup target add wasm32-unknown-unknown
cargo test --manifest-path rust/Cargo.toml
cargo build --release --target wasm32-unknown-unknown --manifest-path rust/Cargo.toml
cp rust/target/wasm32-unknown-unknown/release/netnumerics_core.wasm .
node tests/wasm.test.cjs netnumerics_core.wasm
```

On Windows PowerShell replace `cp` with `Copy-Item`. Reload the HTTP-served page and check the engine indicator. Run `node tests/engine.test.cjs` to verify the JS methods and scenarios. Rust compilation and parity testing are configured in CI; the supplied environment did not include Rust, so the prebuilt WASM binary is not included.

## Deploy to GitHub Pages

1. Create a new **public** repository named `netnumerics-98` on GitHub.
2. Upload the extracted files, including `.github/workflows/pages.yml`, or use Git from the project directory:

```sh
git init -b main
git add .
git commit -m "Build NetNumerics 98 numerical security lab"
git remote add origin https://github.com/YOUR-USERNAME/netnumerics-98.git
git push -u origin main
```

3. Repository **Settings → Pages → Build and deployment → Source → GitHub Actions**. Under Actions, run the workflow manually if the first push occurred before Pages was configured. Once deployment succeeds, the expected URL is `https://YOUR-USERNAME.github.io/netnumerics-98/` (replace the placeholder). CI runs the JS and Rust tests, builds WASM, verifies parity and deploys. No public repository or live URL has been created by this package itself.

## Input data

```csv
time_s,events_per_s
0,22
1,24
2,23
```

Example fragment only: a real import needs 9–5000 nonnegative, equally spaced samples. Rates can represent aggregated denied connections per second, failed authentications per second, or another consistently measured event rate. No IP addresses or private traffic are required. The app makes no network measurements itself; the synthetic presets are generated locally.

## Course mapping

Based on the nine uploaded Numerical Methods lecture files, with selected methods (not every subsection):

| Chapter | Numerical method | Purpose |
|---|---|---|
| 1 | Absolute and relative errors | Compare reconstruction against a held-out observation |
| 2 | Taylor expansion, order 0–2 | One-step forecast from prior samples |
| 3 | Bisection, Newton–Raphson | Locate an interpolated threshold crossing |
| 4 | Secant | Compare root convergence |
| 5 | Gaussian elimination with ordinary partial pivoting | Solve least-squares normal equations |
| 6 | Linear / polynomial least squares | Estimate normal baseline |
| 7 | Newton divided differences, Lagrange | Reconstruct a held-out sample |
| 8 | Composite trapezoid and Simpson 1/3 | Integrate positive excess event rate |
| 9 | Central difference, Richardson extrapolation | Estimate rate of change |

### Model

Fit p(i) = c0 + c1*i (+ c2*i²) to the first N samples using least squares and Gaussian elimination. Flag subsequent samples when r(i) = observed(i) − p(i) ≥ threshold. A local quadratic through three residual samples estimates the threshold-crossing time, solved by three root methods. Integration estimates excess events after training, with excess anchored to zero at the final training point. Simpson 1/3 uses a trapezoid tail for an odd interval count. The interpolation experiment hides an observed sample and uses four neighbors. Taylor forecasts from backward-difference derivative estimates. Central differences and Richardson use neighboring observations; these are retrospective estimates, not real-time causal detectors.

### Limitations

- Not an IDS or an attack classifier. A high residual could be benign, and a poor baseline can cause false alerts. No labeled validation dataset is included.
- Synthetic scenarios are explicitly marked. The app supports actual imported aggregate telemetry but does not capture it.
- Polynomial extrapolation and differentiation of noise can be unreliable. The integration-rule difference is not a rigorous error bound. Numerical root agreement is agreement on the fitted model, not proof of the true onset.
- Not implemented: nonlinear systems, scaled pivoting, tridiagonal and Gauss–Jordan solvers, multiple regression, inverse interpolation, Simpson 3/8. The project uses selected methods from each chapter rather than pretending to cover every topic.
- Do not upload course PDFs or private telemetry to the repository.

## Demo script (Arabic)

1. افتح الواجهة واعرض السلوك الطبيعي ثم حالة الارتفاع المستمر. اشرح أن البيانات التجريبية ليست هجوماً حقيقياً.
2. الـ baseline هو الانحدار الخطي أو التربيعي المستنتج من أول 20 عينة. العتبة هي الفرق بين القياس والتوقع.
3. افتح Root solver واشرح كيف تحدد الطرق الثلاث زمن عبور العتبة بين عينتين، مع عرض عدد التكرارات والخطأ المتبقي.
4. افتح Numerical methods: اعرض مصفوفة المعادلات الطبيعية وخطوات Gaussian elimination، ثم التكامل والتفاضل وتجربة إعادة بناء عينة محجوبة.
5. غيّر العتبة أو استورد CSV من قياسات مصرح بها، ثم صدّر التقرير. وضّح أن الإنذار لا يثبت وجود هجوم وإنما يحتاج إلى تحقق من السجلات.

## Files

- `index.html`, `style.css`, `app.js`: retro desktop UI and interaction.
- `engine.js`: standalone reference numerical engine.
- `rust/`: Rust kernels and unit tests.
- `tests/`: numerical and WASM parity checks.
- `.github/workflows/pages.yml`: build and deployment pipeline.
