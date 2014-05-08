//! A resource manager to load meshes.

use std::io::IoResult;
use std::rc::Rc;
use std::cell::RefCell;
use std::local_data;
use collections::HashMap;
use resource::Mesh;
use nprocgen::mesh::MeshDescr;
use nprocgen::mesh;
use loader::obj;
use loader::mtl::MtlMaterial;

local_data_key!(KEY_MESH_MANAGER: MeshManager)

/// The mesh manager.
///
/// Upon construction, it contains:
///
/// It keeps a cache of already-loaded meshes. Note that this is only a cache, nothing more.
/// Thus, its usage is not required to load meshes.
pub struct MeshManager {
    meshes:       HashMap<~str, Rc<RefCell<Mesh>>>
}

impl MeshManager {
    /// Creates a new mesh manager.
    pub fn new() -> MeshManager {
        let mut res = MeshManager {
            meshes: HashMap::new()
        };

        let _ = res.add_mesh_descr(mesh::unit_sphere(50, 50), false, "sphere");
        let _ = res.add_mesh_descr(mesh::unit_cube(), false, "cube");
        let _ = res.add_mesh_descr(mesh::unit_cone(50), false, "cone");
        let _ = res.add_mesh_descr(mesh::unit_cylinder(50), false, "cylinder");

        res
    }

    /// Mutably applies a function to the mesh manager.
    pub fn get_global_manager<T>(f: |&mut MeshManager| -> T) -> T {
        if local_data::get(KEY_MESH_MANAGER, |mm| mm.is_none()) {
            local_data::set(KEY_MESH_MANAGER, MeshManager::new())
        }

        local_data::get_mut(KEY_MESH_MANAGER, |mm| f(mm.unwrap()))
    }

    /// Get a mesh with the specified name. Returns `None` if the mesh is not registered.
    pub fn get(&mut self, name: &str) -> Option<Rc<RefCell<Mesh>>> {
        self.meshes.find(&name.to_owned()).map(|t| t.clone())
    }

    /// Adds a mesh with the specified name to this cache.
    pub fn add(&mut self, mesh: Rc<RefCell<Mesh>>, name: &str) {
        let _ = self.meshes.insert(name.to_owned(), mesh);
    }

    /// Adds a mesh with the specified mesh descriptor and name.
    pub fn add_mesh_descr(&mut self, descr: MeshDescr<f32>, dynamic_draw: bool, name: &str) -> Rc<RefCell<Mesh>> {
        let mesh = Mesh::from_mesh_descr(descr, dynamic_draw);
        let mesh = Rc::new(RefCell::new(mesh));

        self.add(mesh.clone(), name);

        mesh
    }

    /// Removes a mesh from this cache.
    pub fn remove(&mut self, name: &str) {
        self.meshes.remove(&name.to_owned());
    }

    // FIXME: is this the right place to put this?
    /// Loads the meshes described by an obj file.
    pub fn load_obj(path: &Path, mtl_dir: &Path, geometry_name: &str)
                    -> IoResult<Vec<(~str, Rc<RefCell<Mesh>>, Option<MtlMaterial>)>> {
        obj::parse_file(path, mtl_dir, geometry_name).map(|ms| {
            let mut res = Vec::new();

            for (n, m, mat) in ms.move_iter() {
                let m = Rc::new(RefCell::new(m));

                res.push((n, m, mat));
            }

            res
        })
    }
}
