import { readFileSync } from 'fs';

function decodeRow(val: String) {
    return parseInt(val.split('F').join('0').split('B').join('1'), 2);
}

function decodeColumn(val: String) {
    return parseInt(val.split('L').join('0').split('R').join('1'), 2);
}

function calculateId(seat: String) {
    let row = seat.slice(0, 8);
    let column = seat.slice(7);

    return 8 * decodeRow(row) + decodeColumn(column);
}

let path = process.argv[2];
let data = readFileSync(path, 'utf-8').split('\n');

let ids = data.map(calculateId).sort();
let result = ids.find((val, iter) => { return ids[iter + 1] == val + 2 }) + 1;

console.log(result);
