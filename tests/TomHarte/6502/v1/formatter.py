import json

files = ["{:02x}.json".format(opcode) for opcode in range(0, 0x100)]

for file in files:
    output = []
    with open(file, 'r') as f:
        data = json.load(f)

    for entry in data:
        name = entry['name']
        initial_state = entry['initial']
        final_state = entry['final']
        cycles = []
        for cycle in entry['cycles']:
            addr = cycle[0]
            byte = cycle[1]
            operation = cycle[2]
            cycles.append({'address' : addr, 'byte' : byte, 'action' : operation})
        output.append({'name' : name, 'initial_state' : initial_state, 'final_state' : final_state, 'cycles' : cycles})
    output_file = "f_" + file
    with open(output_file, 'w') as f:
        json.dump(output, f)
