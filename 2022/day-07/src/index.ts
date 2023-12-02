import fs from "fs"
import assert from "assert"

class Arquivo {
	name: string
	size: number

	constructor(name: string, size: number) {
		this.name = name
		this.size = size
	}
}

class Directory {
	name: string
	parent: Directory|null
	files: Array<Arquivo>
	directories: Array<Directory>
	size: number

	constructor(name: string, parent: Directory|null) {
		this.name = name
		this.parent = parent
		this.size = 0
		this.files = []
		this.directories = []
	}
}

class Cursor {
	root: Directory|null
	current: Directory|null

	constructor() {
		this.root = null
		this.current = null
	}

	private create_root() {
		this.root = new Directory('/', null)
		this.current = this.root
	}

	private update_size(new_file_size: number) {
		let current = this.current
		assert.ok(current)

		do {
			current.size += new_file_size
			current = current.parent
		} while (current != null)
	}

	cd(path: string) {
		if (path == '/') return this.create_root()
		const current = this.current
		assert.ok(current)
		if (path == "..") {
			this.current = current.parent
		} else {
			const dir = current.directories.find(v => v.name == path)
			assert.ok(dir)
			this.current = dir
		}
	}

	insert_file(filename: string, size: number) {
		const current = this.current
		assert.ok(current)
		const file = new Arquivo(filename, size)
		current.files.push(file)
		this.update_size(size)
	}

	insert_folder(name: string) {
		const current = this.current
		assert.ok(current)
		const folder = new Directory(name, this.current)
		current.directories.push(folder)
	}
}

// const filename = "example.txt"
const filename = "input.txt"

const input = fs.readFileSync(filename, 'utf8').split("\n").filter(v => !!v)

const cursor = new Cursor()

for (const row of input) {
	if (row.startsWith('$')) {
		const [_, command, arg] = row.split(' ')
		if (command == 'cd') {
			cursor.cd(arg)
		} else if (command == 'ls') {
			continue
		}
	} else if (row.startsWith('dir')) {
		const [_, dirname] = row.split(' ')
		cursor.insert_folder(dirname)
	} else {
		const [size, filename] = row.split(' ')
		cursor.insert_file(filename, Number(size))
	}
}

console.dir(cursor.root, { depth: null })

const root_dir = cursor.root
assert.ok(root_dir)

let nodes: Array<Directory> = [root_dir]
const nodes_found: Array<Directory> = []

for (const node of nodes) {
	if (node.size <= 100000) {
		nodes_found.push(node)
	}
	nodes.push(...node.directories)
}

const computed_size = nodes_found.reduce((acc, cur) => acc + cur.size, 0)

console.log('computed_size', computed_size)

const total_space = 70000000
const required_space = 30000000
const minimum_dir_size = Math.abs(total_space - root_dir.size - required_space)
assert(required_space > minimum_dir_size)

console.log('minimum_dir_size', minimum_dir_size, 'total_size', root_dir.size)

const valid_size_nodes: Array<Directory> = []

nodes = [root_dir]

for (const node of nodes) {
	if (node.size >= minimum_dir_size) {
		valid_size_nodes.push(node)
	}
	nodes.push(...node.directories)
}

const minimum_valid_size = valid_size_nodes
	.map(v => v.size)
	.sort((a, b) => a - b)[0]

console.log('minimum_valid_size', minimum_valid_size)
