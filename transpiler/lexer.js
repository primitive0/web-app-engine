const tokens = {
    IDENT: 'IDENT',
    SEP: 'SEP',
    LINE_BREAK: 'LINE_BREAK',
    ASSIGN: 'ASSIGN',
    LITERAL_INTEGER: 'LITERAL_INTEGER',
    PAREN_OPEN: 'PAREN_OPEN',
    PAREN_CLOSE: 'PAREN_CLOSE',
    BRACE_OPEN: 'BRACE_OPEN',
    BRACE_CLOSE: 'BRACE_CLOSE',
    EOF: 'EOF'
}

function isLineBreak(char) {
    return char == '\n' || char == '\r';
}

const LEXER_STATES = {
    'reset': {
        check(buf, char) {
            return false;
        },
        emit(buf) {
        }
    },
    'whitespaces': {
        check(buf, char) {
            return char == ' ' || isLineBreak(char);
        },
        *emit(buf) {
            let lineBreaks;
            let whitespaces = '';
            for (const char of buf) {
                whitespaces += char;
                if (whitespaces.length == 1) {
                    lineBreaks = isLineBreak(char);
                } else {
                    if (lineBreaks && char == ' ' || !lineBreaks && isLineBreak(char)) {
                        yield {id: lineBreaks ? tokens.LINE_BREAK : tokens.SEP, data: whitespaces};
                        whitespaces = '';
                    }
                }
            }
            if (whitespaces.length != 0) {
                yield {id: lineBreaks ? tokens.LINE_BREAK : tokens.SEP, data: whitespaces};
            }
        }
    },
    'integer_literal': {
        check(buf, char) {
            return /\d/.test(char);
        },
        emit(buf) {
            let parsed = +buf;
            if (!parsed) {
                throw Error('failed to parse token');
            }
            return {id: tokens.LITERAL_INTEGER, data: parsed};
        }
    },
    'ident': {
        check(buf, char) {
            return /[A-Za-z]/.test(char) || (buf.length != 0 && /\d/.test(char));
        },
        emit(buf) {
            return {id: tokens.IDENT, data: buf};
        }
    },
    'assign': {
        check(buf, char) {
            return buf.length == 0 && char == '=';
        },
        emit(buf) {
            return {id: tokens.ASSIGN};
        }
    },
    'paren_open': {
        check(buf, char) {
            return buf.length == 0 && char == '(';
        },
        emit(buf) {
            return {id: tokens.PAREN_OPEN};
        }
    },
    'paren_close': {
        check(buf, char) {
            return buf.length == 0 && char == ')';
        },
        emit(buf) {
            return {id: tokens.PAREN_CLOSE};
        }
    },
    'brace_open': {
        check(buf, char) {
            return buf.length == 0 && char == '{';
        },
        emit(buf) {
            return {id: tokens.BRACE_OPEN};
        }
    },
    'brace_close': {
        check(buf, char) {
            return buf.length == 0 && char == '}';
        },
        emit(buf) {
            return {id: tokens.BRACE_CLOSE};
        }
    }
};

class Lexer {
    constructor(contents) {
        this.contents = contents;
        this.i = 0;
        this.state = LEXER_STATES.reset;
        this.buf = '';
        this.tokens = [];
        this.end = false;
    }

    _stateEmit() {
        let result = this.state.emit(this.buf);
        if (!result) {
            result = [];
        } else if (typeof result == 'object' && typeof result.next == 'function') {
            result = [...result];
        } else {
            result = [result];
        }

        const [token, ...left] = result;
        this.tokens.push(...left);

        return token;
    }

    next() {
        if (this.tokens.length != 0) {
            return this.tokens.pop();
        }

        while (this.i < this.contents.length) {
            const char = this.contents.charAt(this.i);
            if (this.state.check(this.buf, char)) {
                this.buf += char;
                this.i++;
            } else {
                const token = this._stateEmit();
                if (token) {
                    this.state = LEXER_STATES.reset;
                    return token;
                } else {
                    let stateFound = false;
                    for (let newState of Object.values(LEXER_STATES)) {
                        if (newState.check('', char)) {
                            this.state = newState;
                            stateFound = true;
                            break;
                        }
                    }
                    if (!stateFound) {
                        throw Error('new tokenizer state not found');
                    }
                    this.buf = char;
                    this.i++;
                }
            }
        }
        const token = this._stateEmit();
        if (token) {
            this.state = LEXER_STATES.reset;
            return token;
        }
        if (!this.end) {
            this.end = true;
            return {id: tokens.EOF};
        }
    }
}

module.exports = {tokens, Lexer};
