const {tokens} = require('./lexer');

function simpleTokenMatcher(matcherId, tokenId) {
    return {
        id: matcherId,
        match(seq) {
            let token;
            while (true) {
                token = seq.next();
                if (!token) {
                    return false;
                }
                if (token.id == tokens.SEP || token.id == tokens.LINE_BREAK) {
                    continue;
                }
                return token.id == tokenId && token;
            }
        }
    };
}

const tm = {
    IDENT: simpleTokenMatcher('IDENT', tokens.IDENT),
    ASSIGN: simpleTokenMatcher('ASSIGN', tokens.ASSIGN),
    LITERAL_INTEGER: simpleTokenMatcher('LITERAL_INTEGER', tokens.LITERAL_INTEGER),
    PAREN_OPEN: simpleTokenMatcher('PAREN_OPEN', tokens.PAREN_OPEN),
    PAREN_CLOSE: simpleTokenMatcher('PAREN_CLOSE', tokens.PAREN_CLOSE),
    BRACE_OPEN: simpleTokenMatcher('BRACE_OPEN', tokens.BRACE_OPEN),
    BRACE_CLOSE: simpleTokenMatcher('BRACE_CLOSE', tokens.BRACE_CLOSE),
    CBREAK: {
        id: 'STATEMENT_END',
        match(seq) {
            let token = seq.next();
            if (token.id == tokens.LINE_BREAK || token.id == tokens.EOF) {
                return token;
            }
        }
    }
};

const MATCHERS = {
    'function_declaration': {
        pattern: [tm.IDENT, tm.IDENT, tm.PAREN_OPEN, tm.PAREN_CLOSE, tm.BRACE_OPEN, tm.BRACE_CLOSE, tm.CBREAK],
        emit(tokens) {
            return {id: 'function_declaration'};
        }
    },
    'variable_declaration': {
        pattern: [tm.IDENT, tm.IDENT, tm.ASSIGN, tm.LITERAL_INTEGER, tm.CBREAK],
        emit(tokens) {
            let type = tokens[0].name;
            let name = tokens[1].name;
            let value = tokens[3].value;
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

function matchPattern(seq, pattern) {
    let matched = [];

    for (let pm of pattern) {
        let token = pm.match(seq);
        if (!token) {
            return false;
        }
        matched.push(token);
    }

    return matched;
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
