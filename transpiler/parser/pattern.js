const {tokens} = require('../lexer');

const TOKEN_MATCHERS = {
    IDENT: {id: 'IDENT', matches: tokens.IDENT},
    ASSIGN: {id: 'ASSIGN', matches: tokens.ASSIGN},
    LITERAL_INTEGER: {id: 'LITERAL_INTEGER', matches: tokens.LITERAL_INTEGER},
    PAREN_OPEN: {id: 'PAREN_OPEN', matches: tokens.PAREN_OPEN},
    PAREN_CLOSE: {id: 'PAREN_CLOSE', matches: tokens.PAREN_CLOSE},
    BRACE_OPEN: {id: 'BRACE_OPEN', matches: tokens.BRACE_OPEN},
    BRACE_CLOSE: {id: 'BRACE_CLOSE', matches: tokens.BRACE_CLOSE},
    CBREAK: {id: 'CONSTRUCTION_BREAK'}
};

const tm = {};

function copyTm(tm) {
    return {
        id: tm.id,
        matches: tm.matches,
        _tokenRequired: tm._tokenRequired,
    }
}

for (const [id, matcher] of Object.entries(TOKEN_MATCHERS)) {
    if (tm.id == TOKEN_MATCHERS.CBREAK.id) {
        tm[id] = matcher;
        continue;
    }

    tm[id] = {
        id: matcher.id,
        matches: matcher.matches,
        _tokenRequired: false,
        get req() {
            const copy = copyTm(this);
            copy._tokenRequired = true;
            return copy;
        }
    };
}

function matchToken(seq, tokenId) {
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

function matchPattern(seq, pattern) {
    let matched = [];

    for (let tm of pattern) {
        let token;
        if (tm.id == TOKEN_MATCHERS.CBREAK.id) {
            token = seq.next();
            if (!(token.id == tokens.LINE_BREAK || token.id == tokens.EOF)) {
                token = null;
            }
        } else {
            token = matchToken(seq, tm.matches);
        }

        if (!token) {
            return false;
        }
        if (tm._tokenRequired) {
            matched.push(token.data);
        }
    }

    return matched;
}

module.exports = {tm, matchPattern};
