
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
 * @returns {Promise<Uint8Array|null>}
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

    refresInputContents();
}

const resetDecodeBoxContent = () => {
    decodeResultBoxElement.hidden = true;
    secretMessageBoxElement.textContent = "";

    refresInputContents();
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

/**
 * Get an image from the given URL
 * 
 * @param {string} url
 * @returns {Promise<Uint8Array|null>}
 */
const curlImage = async (url) => {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Failed to fetch the image: ${response.status}`);
        }

        const blob = await response.blob();
        const rawBytes = await blob.arrayBuffer();
        return new Uint8Array(rawBytes);
    } catch (error) {
        alert(`Failed to download image from URLL: ${url}\n...${error.message}`)
        return null;
    }
}

/**
 * Return the code representing which image source (URL or uploading) should be used
 * @returns {number}
 * - 0: both are empty (00 in binary)
 * - 1: only URL is provided (01 in binary)
 * - 2: only uploading is provided (10 in binary)
 * - 3: both are provided (11 in binary)
 */
const urlOrUpload = () => {
    const urlBit = pngURLInput.value ? 1 : 0;
    const uploadingBit = uploadInput.files[0] ? 1 : 0;

    return (uploadingBit << 1) | urlBit;
}

// Handle submitting form
const pngmeForm = document.getElementById('pngmeForm');
pngmeForm.addEventListener('submit', async (event) => {
    try {
        event.preventDefault(); // Prevent default form submission

        let fileBytes = null;
        let filename = null;
        const imageSourceCode = urlOrUpload();

        switch (imageSourceCode) {
            case 0:
                throw new Error("Please upload your PNG image OR paste the URL linking to it");
            case 1:
                // URL is used
                // Currently is not supported
                // const url = pngURLInput.value;
                // fileBytes = await curlImage(url);
                // filename = util.getFilename(url);
                throw new Err("This option is currently not supported!!!");
            case 2:
                // File is uploaded
                fileBytes = await readUploadFile();
                filename = uploadInput.files[0].name;
                break;
            case 3:
                throw new Err("Please explicitly upload your file OR link the URL, don't use both");
            default:
                throw new Err("Invalid image source option");
        }

        const passphrase = passphraseInput.value;
        if (util.isEmpty(passphrase)) {
            throw new Err("Please enter the passphrase!");
        }
        const chunkType = chunkTypeField.value;
        if (util.isEmpty(chunkType)) {
            throw new Err("Please enter the chunk type");
        }
        if (chunkType.length !== 4) {
            throw new Err("Your chunk type is not exactly 4-bytes long");
        }

        const opertaionMode = document.querySelector('input[name="opMode"]:checked').value;

        if (opertaionMode === 'encode') {
            const message = messageField.value;
            if (util.isEmpty(message)) {
                throw new Err("Please enter your message");
            }

            // Start encode
            const publicMaterial = pngme.encode(fileBytes, passphrase, message, chunkType);
            const nonce = publicMaterial.nonce;
            const encodedBytes = publicMaterial.encoded_bytes;

            resetDecodeBoxContent();

            // Display results
            displayImage(encodedBytes, filename);
            nonceValueElement.textContent = nonce;
            encodeResultBoxElement.hidden = false;

        } else if (opertaionMode === 'decode') {
            const nonce = nonceInput.value;
            if (util.isEmpty(nonce)) {
                throw new Err("Please enter your message");
            }
            // Start decode
            console.log("hihihi")

            const plaintext = pngme.decode(fileBytes, passphrase, nonce, chunkType);
            if (!plaintext) {
                throw new Err("Something wrong in decoding");
            }

            resetEncodeBoxContent();

            // Display results
            decodeResultBoxElement.hidden = false;
            secretMessageBoxElement.textContent = plaintext;
        }

    } catch (error) {
        alert(`Error: ${error.message}`);
        return;
    }
});