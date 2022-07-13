const {tokens} = require('../lexer');
const {tm, matchPattern} = require('./pattern');

const MATCHERS = {
    'function_declaration': {
        pattern: [tm.IDENT, tm.IDENT, tm.PAREN_OPEN, tm.PAREN_CLOSE, tm.BRACE_OPEN, tm.BRACE_CLOSE, tm.CBREAK],
        emit(tokens) {
            return {id: 'function_declaration'};
        }
    },
    'variable_declaration': {
        pattern: [tm.IDENT.req, tm.IDENT.req, tm.ASSIGN, tm.LITERAL_INTEGER.req, tm.CBREAK],
        emit([type, name, value]) {
            return {id: 'variable_declaration', type, name, value};
        }
    }
};

class TokenSeq {
    constructor(lexer) {
        this.lexer = lexer;
        this.tokens = [];
        this.pos = 0;
    }

    pop() {
        this.tokens = [];
        this.pos = 0;
    }

    reset() {
        this.pos = 0;
    }

    seek() {
        let token;
        if (this.pos == this.tokens.length) {
            token = this.lexer.next();
            if (!token) {
                return null;
            }
            this.tokens.push(token);
        } else {
            token = this.tokens[this.pos];
        }
        return token;
    }

    skip() {
        this.pos++;
    }

    next() {
        let token = this.seek();
        this.pos++;
        return token;
    }
}

function buildAST(lexer) {
    const ast = [];

    const seq = new TokenSeq(lexer);

    let nextToken = seq.seek();
    while (nextToken && nextToken.id != tokens.EOF) {
        let matched;
        for (const matcher of Object.values(MATCHERS)) {
            matched = matchPattern(seq, matcher.pattern);
            if (matched) {
                let astNode = matcher.emit(matched);
                ast.push(astNode);
                seq.pop();
                break;
            } else {
                seq.reset();
            }
        }
        if (!matched) {
            throw Error('ast building error');
        }

        nextToken = seq.seek();
    }

    return ast;
}

module.exports = {buildAST};
