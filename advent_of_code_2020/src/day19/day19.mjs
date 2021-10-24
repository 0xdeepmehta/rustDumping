import fs from 'fs';

function parseRule(rhs) {
    if (rhs.indexOf('a') > -1) {
        return 'a';
    }

    if (rhs.indexOf('b') > -1) {
        return 'b';
    }

    return rhs.split(" | ").map(v => v.split(" "));
}

function patchRules(rules) {
    const patched = rules
        .split('\n')
        .filter(r => r.indexOf('8:') === -1 || r.indexOf('11:') === -1);

    patched.push('8: 42 | 42 8');
    patched.push('11: 42 31 | 42 11 31');

    return patched.join('\n');
}

function parseTree(rulesStr) {
    const rules = {};

    rulesStr.split('\n')
        .forEach(r => {
            const [index, rule] = r.split(': ');
            rules[index] = parseRule(rule);
        });

    return rules;
}

function test(str, todo) {
    if (todo.length === 0) {
        return str === '';
    }

    const next = todo[0];
    const others = todo.slice(1);

    const descend = rules[next];
    if (Array.isArray(descend)) {
        return descend.some(r => test(str, [...r, ...others]));
    } else {
        return str[0] === descend && test(str.slice(1), others);
    }
}

const raw = fs.readFileSync('./data/input.txt', 'utf-8');
const [rulesStr, inputStr] = raw.split('\n\n');

const patchedRules = patchRules(rulesStr);
const rules = parseTree(patchedRules);

const input = inputStr.split('\n');

const mainRule = rules['0'][0];
const results = input.map(i => test(i, mainRule)).filter(v => v).length;

console.log(results);
