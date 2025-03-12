To test the server from a fresh git pull:
- Install Node.js
- Install Rust (1.80.1)
* Invoke-WebRequest -Uri https://sh.rustup.rs -OutFile rustup-init.exe; .\rustup-init.exe
* rustup install 1.80.1
- Navigate to the server directory (`cd server`)
- Run `npx workers dev`
This starts a development testing server on the local device using port 8787.

To test the client:
- Install Webpack using Node.js
- Pack the client using `npx webpack`
- Open the packed file at `frontsend/dist/<INDEX FILE>`

---

Theoretically, the server worker may be deployed to cloudflare servers using the deploy command (for example `npx wrangler deploy`), though this has not been tested.
The frontend itself should be completely static, and can be served via any typical webserver. Cloudflare already provides a service for this in the form of "Static Assets", though this has also not been tested in this case.