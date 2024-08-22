/**
 * Check if the given string is empty (whitespaces are excluded)
 * 
 * @param {string} str 
 * @returns {number}
 */
const isEmpty = (str) => {
    return !str.trim().length;
};

/**
 * Get a file name from a full file name
 * E.g: abc.mp3.img -> 'abc'
 * 
 * @param {string} fullFileName
 * @return {string} filename
 */
const getFilename = (fullFileName) => {
    let name = fullFileName;
    while (name && name.includes('.')) {
        name = name.split('.').at(0);
    }
    return name;
}

export { isEmpty, getFilename };