from jinja2 import Environment

def make_dot_mapping():
    res = {}
    for i in range(1, 64):
        dots = []
        for j in range(8):
            if i & (1 << j):
                dots.append(str(j + 1))
        res[i] = ''.join(dots)
    return res

if __name__ == '__main__':
    with open('pattern.rs.gen', 'r') as f:
        template = f.read()
    env = Environment()
    res = env.from_string(template).render({'patterns': make_dot_mapping()})
    with open('pattern.rs', 'w') as f2:
        f2.write(res)