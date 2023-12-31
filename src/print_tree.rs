use crate::doublons;
use crate::file_tree::FileTree;
use crate::size::Size;
use std::path::{Path, PathBuf};

// Fonction pour afficher un nœud
fn print_node(node_path: &Path, depth: usize, size: Option<&Size>) {
    let node_name = node_path.to_path_buf();

    // Indentation pour montrer la hiérarchie
    print!("{:indent$}", "", indent = depth * 2);

    // Affichage de la taille si le nœud est un fichier
    if let Some(size) = size {
        print!("{}", size);
    }

    // Affichage du nom du nœud
    println!(" {}", node_name.to_string_lossy());
}

fn display_duplacate_files(file_tree: &FileTree, filter_extension: Option<&str>) {
    match doublons::find_duplicate_files(file_tree) {
        Ok(contents) => {
            for content in contents {
                let mut cmp: u32 = 0;
                for file in content {
                    if let Some(extension) = filter_extension {
                        let path_try: PathBuf = PathBuf::from(&file);
                        // println!("path_try {:?}, extension {:?}", &path_try, extension);
                        if filter_child(&path_try, extension) {
                            println!("{:?}", file);
                            cmp += 1;
                        }
                    } else {
                        println!("{:?}", file);
                        cmp += 1;
                    }
                }
                if cmp != 0 {
                    println!();
                }
            }
        }
        Err(err) => eprintln!("Erreur de lecture du fichier : {}", err),
    }
}

// Définir la fonction de filtrage
fn filter_child(child: &Path, extension: &str) -> bool {
    if let Some(file_name) = child.file_name() {
        if let Some(child_extension) = file_name.to_str().and_then(|s| Path::new(s).extension()) {
            if let Some(child_extension) = child_extension.to_str() {
                return ".".to_string() + child_extension == extension;
            }
        }
    }
    false
}

// Fonction pour filtrer les enfants par extension
fn filter_children(
    file_tree: &FileTree,
    children: Vec<PathBuf>,
    filter_extension: Option<&str>,
    lexicographic_sort: bool,
) {
    if let Some(extension) = filter_extension {
        let mut vec_children: Vec<PathBuf> = children
            .into_iter()
            .filter(|child| filter_child(child, extension))
            .collect();
        sort_children(&mut vec_children, file_tree, lexicographic_sort);
        for child in vec_children {
            print_node(&child, 0, file_tree.get_size(&child).as_ref());
        }
    }
}

// Fonction pour trier les enfants
fn sort_children(children: &mut [PathBuf], file_tree: &FileTree, lexicographic_sort: bool) {
    // Trier les enfants selon l'option de tri
    if lexicographic_sort {
        children.sort();
    } else {
        children.sort_by(|a, b| {
            file_tree
                .get_size(b)
                .unwrap()
                .cmp(&file_tree.get_size(a).unwrap())
        });
    }
}

// Fonction auxiliaire récursive pour afficher un nœud et ses enfants
fn display_node_recursive(
    file_tree: &FileTree,
    node_path: &Path,
    depth: usize,
    lexicographic_sort: bool,
) {
    let size = file_tree.get_size(node_path);

    print_node(node_path, depth, size.as_ref());

    // Affichage des enfants si le nœud est un répertoire
    if let Some(children) = file_tree.get_children(node_path) {
        let mut sorted_children = children.to_vec();

        sort_children(&mut sorted_children, file_tree, lexicographic_sort);

        for child_path in sorted_children {
            display_node_recursive(file_tree, &child_path, depth + 1, lexicographic_sort);
        }
    }
}

impl FileTree {
    pub fn show(
        &self,
        lexicographic_sort: bool,
        filter_extension: Option<&str>,
        is_duplicate: bool,
    ) {
        if is_duplicate {
            display_duplacate_files(self, filter_extension);
        } else if filter_extension.is_some() {
            let children = self.files();
            filter_children(self, children, filter_extension, lexicographic_sort);
        } else {
            // Affichage du contenu de l'arbre, en commençant par la racine
            display_node_recursive(self, self.get_root(), 0, lexicographic_sort);
        }
    }
}
