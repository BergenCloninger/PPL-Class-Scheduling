async function generateRSAKeyPair() {
    return await crypto.subtle.generateKey(
        {
            name: "RSA-OAEP",
            modulusLength: 2048,
            publicExponent: new Uint8Array([1, 0, 1]),
            hash: "SHA-256",
        },
        true,
        ["encrypt", "decrypt"]
    );
}

async function exportPublicKey(key) {
    const publicKey = await crypto.subtle.exportKey("spki", key);
    return arrayBufferToBase64(publicKey);
}

function arrayBufferToBase64(buffer) {
    const bytes = new Uint8Array(buffer);
    return btoa(String.fromCharCode(...bytes));
}

async function encryptFields(payload, sensitiveFields, publicKey) {
    const encryptedPayload = { ...payload };

    for (const field of sensitiveFields) {
        if (payload[field]) {
            const encoder = new TextEncoder();
            const encodedData = encoder.encode(payload[field]);

            const encryptedData = await crypto.subtle.encrypt(
                {
                    name: "RSA-OAEP"
                },
                publicKey,
                encodedData
            );

            encryptedPayload[field] = arrayBufferToBase64(encryptedData);
        }
    }

    return encryptedPayload;
}

async function sendRequest({ apiEndpoint, inputMethod, inputBody = null}) {
    const response = await fetch(apiEndpoint, {
        method: inputMethod,
        headers: {
            "Content-Type": "text/plain"
        },
        body: inputBody,
    });

    let responseData = null;

    if (response.ok && response.body) {
        const text = await response.text();
        if (text) {
            try {
                responseData = JSON.parse(text);
            } catch (error) {
                console.warn('Failed to parse response as JSON:', error);
            }
        }
    }

    return responseData;
}

document.getElementById("run").addEventListener("click", async () => {
    const outputElement = document.getElementById("output");

    try {
        const user = { username: "test4", password: "1234", role: "admin" };

        const path = "http://localhost:8787/create/user"; //TODO: make url constant

        const responseData = await sendRequest({ apiEndpoint: path, inputMethod: "POST", inputBody: JSON.stringify(user) });

        if (responseData && responseData.success) {
            outputElement.textContent += `Create User Response: ${JSON.stringify(responseData.data)}\n`;
        } else {
            outputElement.textContent += `Error: ${responseData.error || 'Empty Response!'}\n`;
        }
    } catch (error) {
        console.error(`Error during request: ${error.message}`);
        outputElement.textContent = `Error: ${error.message}`;
    }
});

document.getElementById("readUser").addEventListener("click", async () => {
    const outputElement = document.getElementById("output");

    try {
        const path = "http://localhost:8787/read/user?1";

        const responseData = await sendRequest({ apiEndpoint: path, inputMethod: "GET"});
        console.log(responseData);
        if (responseData) {
            outputElement.textContent += `Read User Response: ${JSON.stringify(responseData)}\n`;
        } else {
            outputElement.textContent += `Error: ${responseData.error || 'Empty Response!'}\n`;
        }
    } catch (error) {
        console.error(`Error during request: ${error.message}`);
        outputElement.textContent = `Error: ${error.message}`;x
    }
});