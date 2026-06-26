import sys

file_path = 'rustapi/src/routing.rs'
with open(file_path, 'r') as f:
    lines = f.readlines()

new_lines = []
in_broken_impl = False
for line in lines:
    if "            \"PUT\" => Box::new(move |router, path| router.route(path, axum::routing::put(handler)))," in line:
        in_broken_impl = True
        continue
    if in_broken_impl:
        if line.strip() == "}" or line.strip() == "};":
             # We might have multiple closing braces to skip
             continue
        if "pub struct APIRouter" in line or "pub trait Routable" in line:
             in_broken_impl = False
        else:
             continue

    if line.strip() == "}":
        # Check if it's a stray brace
        if len(new_lines) > 0 and new_lines[-1].strip() == "":
             continue

    new_lines.append(line)

with open(file_path, 'w') as f:
    f.writelines(new_lines)
