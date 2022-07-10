const {tokens} = require('./lexer');

function simpleTokenMatcher(matcherId, tokenId) {
    return {
        id: matcherId,
        match(seq) {
            let token = seq.next();
            return token.id == tokenId && token;
        }
    };
}

function lineEnd(matcher) {
    return {
        id: 'LINE_MAY_BREAK',
        match(seq) {
            let token = seq.seek();
            if (token.id == tokens.LINE_BREAK) {
                seq.skip();
            }
            return matcher.match(seq);
        }
    }
}

const tm = {
    LINE_BREAK: simpleTokenMatcher('LINE_BREAK', tokens.LINE_BREAK),
    IDENT: simpleTokenMatcher('IDENT', tokens.IDENT),
    ASSIGN: simpleTokenMatcher('ASSIGN', tokens.ASSIGN),
    LITERAL_INTEGER: simpleTokenMatcher('LITERAL_INTEGER', tokens.LITERAL_INTEGER),
    PAREN_OPEN: simpleTokenMatcher('PAREN_OPEN', tokens.PAREN_OPEN),
    PAREN_CLOSE: simpleTokenMatcher('PAREN_CLOSE', tokens.PAREN_CLOSE),
    BRACE_OPEN: simpleTokenMatcher('BRACE_OPEN', tokens.BRACE_OPEN),
    BRACE_CLOSE: simpleTokenMatcher('BRACE_CLOSE', tokens.BRACE_CLOSE),
    SEND: {
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
        pattern: [tm.IDENT, tm.IDENT, tm.PAREN_OPEN, tm.PAREN_CLOSE, lineEnd(tm.BRACE_OPEN), lineEnd(tm.BRACE_CLOSE)],
        emit(tokens) {
            return {id: 'function_declaration'};
        }
    },
    'variable_declaration': {
        pattern: [tm.IDENT, tm.IDENT, tm.ASSIGN, tm.LITERAL_INTEGER, tm.SEND],
        emit(tokens) {
            // T a = 1
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
            while (true) {
                token = this.lexer.next();
                if (token == null) {
                    return null;
                } else if (token.id == tokens.SEP) {
                    continue;
                }
                this.tokens.push(token);
                break;
            }
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
            seq.reset();
            return false;
        }
        matched.push(token);
    }

    return matched;
}

function buildAST(lexer) {
    const ast = [];

    const seq = new TokenSeq(lexer);

    let nextToken;
    while ((nextToken = seq.seek()).id != tokens.EOF) {
        if (nextToken.id == tokens.LINE_BREAK) {
            seq.skip();
            seq.pop();
        } else {
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
        }
    }

    return ast;
}

module.exports = {buildAST};
