use itertools::Itertools;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct VectorField {
    name: String,
}

struct VectorClass {
    name: String,
    fields: Vec<VectorField>,
    fields_type_name: String,
}

fn main() {
    let classes = vec![
        VectorClass {
            name: String::from("Vector2"),
            fields: vec![
                VectorField {
                    name: String::from("x"),
                },
                VectorField {
                    name: String::from("y"),
                },
            ],
            fields_type_name: String::from("float"),
        },
        // VectorClass {
        //     name: String::from("Vector2Int"),
        //     fields: vec![
        //         VectorField {
        //             name: String::from("x"),
        //         },
        //         VectorField {
        //             name: String::from("y"),
        //         },
        //     ],
        //     fields_type_name: String::from("int"),
        // },
        // VectorClass {
        //     name: String::from("Vector3"),
        //     fields: vec![
        //         VectorField {
        //             name: String::from("x"),
        //         },
        //         VectorField {
        //             name: String::from("y"),
        //         },
        //         VectorField {
        //             name: String::from("z"),
        //         },
        //     ],
        //     fields_type_name: String::from("float"),
        // },
        // VectorClass {
        //     name: String::from("Vector3Int"),
        //     fields: vec![
        //         VectorField {
        //             name: String::from("x"),
        //         },
        //         VectorField {
        //             name: String::from("y"),
        //         },
        //         VectorField {
        //             name: String::from("z"),
        //         },
        //     ],
        //     fields_type_name: String::from("int"),
        // },
    ];

    for class in &classes {
        let fields = class.fields.clone();
        let mut fields_with_extras = fields.clone();
        fields_with_extras.push(VectorField {
            name: String::from("0"),
        });
        fields_with_extras.push(VectorField {
            name: String::from("1"),
        });
        let p = VectorField {
            name: String::from("p"),
        };
        fields_with_extras.push(p.clone());

        for combinations in fields_with_extras
            .iter()
            .combinations_with_replacement(2)
            .unique()
        {
            for permutation in combinations.iter().permutations(2).unique() {
                let mut string = format!("public static {} _", &class.name);
                for field in &permutation {
                    string.push_str(field.name.as_str());
                }
                string.push_str(format!("(this {0} self", &class.name).as_str());
                if permutation.contains(&&&p) {
                    string.push_str(format!(", {0} {1}", class.fields_type_name, p.name).as_str());
                }
                string.push_str(format!(") => new {0}(", &class.name).as_str());
                for field in &permutation {
                    if field.name == "0" {
                        if class.fields_type_name == "float" {
                            string.push_str("0f, ");
                        } else {
                            string.push_str("0, ");
                        }
                    } else if field.name == "1" {
                        if class.fields_type_name == "float" {
                            string.push_str("1f, ");
                        } else {
                            string.push_str("1, ");
                        }
                    } else if field.name == "p" {
                        string.push_str("p, ");
                    } else {
                        string.push_str(format!("self.{}, ", field.name).as_str());
                    }
                }
                string.pop();
                string.pop();
                string.push_str(");");
                println!("{}", string);
            }
        }

        for combinations in fields_with_extras
            .iter()
            .combinations_with_replacement(3)
            .unique()
        {
            for permutation in combinations.iter().permutations(3).unique() {
                let mut string = format!("public static {} _", &class.name);
                for field in &permutation {
                    string.push_str(field.name.as_str());
                }
                string.push_str(format!("(this {0} self", &class.name).as_str());
                if permutation.contains(&&&p) {
                    string.push_str(format!(", {0} {1}", class.fields_type_name, p.name).as_str());
                }
                string.push_str(format!(") => new {0}(", &class.name).as_str());
                for field in &permutation {
                    if field.name == "0" {
                        if class.fields_type_name == "float" {
                            string.push_str("0f, ");
                        } else {
                            string.push_str("0, ");
                        }
                    } else if field.name == "1" {
                        if class.fields_type_name == "float" {
                            string.push_str("1f, ");
                        } else {
                            string.push_str("1, ");
                        }
                    } else if field.name == "p" {
                        string.push_str("p, ");
                    } else {
                        string.push_str(format!("self.{}, ", field.name).as_str());
                    }
                }
                string.pop();
                string.pop();
                string.push_str(");");
                println!("{}", string);
            }
        }
    }
}
