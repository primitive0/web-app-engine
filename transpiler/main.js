const fileToParse = process.argv[2];
const outFile = process.argv[3];
if (!fileToParse) {
    console.log('No file input');
    process.exit(-1);
}
if (!outFile) {
    console.log('No out file');
    process.exit(-1);
}

console.log(`Parsing: ${fileToParse}`);

const fs = require('fs');
const {tokens, Lexer} = require('./lexer');
const {buildAST} = require('./parser');

const fileContents = fs.readFileSync(fileToParse, 'utf8');

function generateOutC(ast) {
    let typeMapping = {
        'int': 'int32_t'
    }

    let mainFunctionLines = [];

    for (const stmt of ast) {
        if (stmt.id == 'variable_declaration') {
            let cType = typeMapping[stmt.type];
            if (!cType) {
                throw Error('failed to map type');
            }

            mainFunctionLines.push(`${cType} ${stmt.name} = ${stmt.value};`);
        }
    }

    let code = '#include <stdint.h>\n' +
        '\n' +
        'int main() {\n';

    for (let line of mainFunctionLines) {
        code += ('    ' + line + '\n');
    }
    code += '\n';
    code += '    return 0;\n';
    code += '}\n';

    return code;
}

// let lexer = new Lexer(fileContents);
// let token;
// while ((token = lexer.next()) != null) {
//     console.log(token);
// }


let lexer = new Lexer(fileContents);
let ast = buildAST(lexer);
let code = generateOutC(ast);

fs.writeFileSync(outFile, code);
