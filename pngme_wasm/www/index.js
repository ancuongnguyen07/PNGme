
import * as pngme from "pngme_wasm";
import * as util from "./js/utils.js";

const encryptRadio = document.getElementById('encryptRadio');
const decryptRadio = document.getElementById('decryptRadio');
const messageField = document.getElementById('message');
const chunkTypeField = document.getElementById('chunkType');
const pngURLInput = document.getElementById('url');
const uploadInput = document.getElementById('uploadInput');
const passphraseInput = document.getElementById('passphrase');
const nonceInput = document.getElementById('nonceInput');

const resetableFields = document.getElementsByClassName("shouldReset");

const nonceValueElement = document.getElementById('nonceValue');
const outputImageElement = document.getElementById('outputImage');
const encodeResultBoxElement = document.getElementById('encodeResultBox');
const decodeResultBoxElement = document.getElementById('decodeResultBox');
const secretMessageBoxElement = document.getElementById('hiddenMessage');
const downloadLinkElement = document.getElementById('downloadLink');

// Toggle the message and chunk type field according to the chosen
// operation mode (encode or decode)
const toggleField = () => {
    if (encryptRadio.checked) {
        messageField.hidden = false;

        nonceInput.hidden = true;
        nonceInput.value = "";


    } else if (decryptRadio.checked) {
        messageField.hidden = true;
        messageField.value = "";

        nonceInput.hidden = false;
    }

    resetDecodeBoxContent();
    resetEncodeBoxContent();
};

encryptRadio.addEventListener('change', toggleField);
decryptRadio.addEventListener('change', toggleField);

// Handle upload file
/**
 * Read the uploaded file and return file byte array.
 * 
 * @returns {Promise<Uint8Array>}
 */
const readUploadFile = () => {
    return new Promise((resolve, reject) => {
        const file = uploadInput.files[0];
        if (file) {
            const reader = new FileReader();

            reader.onload = (e) => {
                const arrayBuffer = e.target.result;
                // Convert to byte array
                const byteArray = new Uint8Array(arrayBuffer);
                resolve(byteArray);
            };

            reader.onerror = reject;
            reader.readAsArrayBuffer(file);
        } else {
            resolve(null);
        }
    });
}

const resetEncodeBoxContent = () => {
    encodeResultBoxElement.hidden = true;
    nonceValueElement.textContent = "";
    outputImageElement.src = "#";
    downloadLinkElement.download = "";
}

const resetDecodeBoxContent = () => {
    decodeResultBoxElement.hidden = true;
    secretMessageBoxElement.textContent = "";
}

/**
 * Refresh all content of input fields when an operation mode is changed
 */
const refresInputContents = () => {
    for (let index = 0; index < resetableFields.length; index++) {
        const element = resetableFields[index];
        element.value = "";
    }
}

/**
 * Displays the PNG image from raw bytes
 * 
 * @param {Uint8Array} data
 * @param {string} filename
 * 
 */
const displayImage = (data, filename) => {
    const blob = new Blob([data], { type: "image/png" });
    const imageURL = URL.createObjectURL(blob);
    outputImageElement.src = imageURL;

    // Provide a download link
    downloadLinkElement.href = imageURL;
    // Name without the extension
    let name = util.getFilename(filename);
    name = `${name}_encoded.png`;
    // Specify the download filename
    downloadLinkElement.download = name;
}

// Handle submitting form
const pngmeForm = document.getElementById('pngmeForm');
pngmeForm.addEventListener('submit', async (event) => {
    event.preventDefault(); // Prevent default form submission

    const fileBytes = await readUploadFile();
    if (fileBytes === null) {
        alert("Please upload a PNG image!!!");
        return;
    }

    const passphrase = passphraseInput.value;
    if (util.isEmpty(passphrase)) {
        alert("Please enter the passphrase!");
        return;
    }
    const chunkType = chunkTypeField.value;
    if (util.isEmpty(chunkType)) {
        alert("Please enter the chunk type");
        return;
    }
    if (chunkType.length !== 4) {
        alert("Your chunk type is not exactly 4-bytes long");
        return;
    }

    const opertaionMode = document.querySelector('input[name="opMode"]:checked').value;
    const filename = uploadInput.files[0].name;
    if (opertaionMode === 'encode') {
        const message = messageField.value;
        if (util.isEmpty(message)) {
            alert("Please enter your message");
            return;
        }

        // Start encode
        try {
            const publicMaterial = pngme.encode(fileBytes, passphrase, message, chunkType);
            const nonce = publicMaterial.nonce;
            const encodedBytes = publicMaterial.encoded_bytes;

            resetDecodeBoxContent();

            // Display results
            displayImage(encodedBytes, filename);
            nonceValueElement.textContent = nonce;
            encodeResultBoxElement.hidden = false;
        } catch (error) {
            alert(`Error Encoding: ${error.message}`);
            return;
        }
    } else if (opertaionMode === 'decode') {
        const nonce = nonceInput.value;
        if (util.isEmpty(nonce)) {
            alert("Please enter your message");
            return;
        }
        // Start decode
        try {
            console.log("hihihi")

            const plaintext = pngme.decode(fileBytes, passphrase, nonce, chunkType);
            if (!plaintext) {
                alert("Something wrong in decoding");
                return;
            }

            resetEncodeBoxContent();

            // Display results
            decodeResultBoxElement.hidden = false;
            secretMessageBoxElement.textContent = plaintext;
        } catch (error) {
            alert(`Error Decoding: ${error.message}`);
        }
    }
});