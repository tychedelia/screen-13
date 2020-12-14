use {
    super::{Data, Lease},
    crate::{
        math::{Quat, Sphere},
        pak::{model::Mesh, IndexType},
    },
    std::{
        cell::{Ref, RefCell, RefMut},
        fmt::{Debug, Error, Formatter},
    },
};

// TODO: Could not force the lifetime to work without an explicit function which means I'm missing something really basic
#[inline]
fn deref_str<S: AsRef<str>>(s: &Option<S>) -> Option<&str> {
    if let Some(s) = s {
        Some(s.as_ref())
    } else {
        None
    }
}

pub struct MeshIter<'a> {
    filter: Option<MeshFilter>,
    idx: usize,
    model: &'a Model,
}

impl<'a> Iterator for MeshIter<'a> {
    type Item = &'a Mesh;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(filter) = self.filter {
            if let Some(mesh) = self.model.meshes.get(filter.0 as usize + self.idx) {
                if mesh.name() == self.model.meshes[self.idx].name() {
                    self.idx += 1;
                    return Some(mesh);
                }
            }

            None
        } else if let Some(mesh) = self.model.meshes.get(self.idx) {
            self.idx += 1;
            Some(mesh)
        } else {
            None
        }
    }
}

/// A reference to an individual mesh name, which may be shared by multiple meshes. Only useful with the Model it was received from.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct MeshFilter(u16);

/// A drawable collection of individually adressable meshes.
pub struct Model {
    idx_buf: RefCell<Lease<Data>>,
    idx_ty: IndexType,
    meshes: Vec<Mesh>,
    pending_writes: RefCell<Option<(u64, u64)>>,
    vertex_buf: RefCell<Lease<Data>>,
}

impl Model {
    /// Meshes must be sorted by name
    pub(crate) fn new(
        meshes: Vec<Mesh>,
        idx_ty: IndexType,
        idx_buf: Lease<Data>,
        idx_buf_len: u64,
        vertex_buf: Lease<Data>,
        vertex_buf_len: u64,
    ) -> Self {
        let pending_writes = RefCell::new(Some((idx_buf_len, vertex_buf_len)));

        Self {
            idx_buf: RefCell::new(idx_buf),
            idx_ty,
            meshes,
            pending_writes,
            vertex_buf: RefCell::new(vertex_buf),
        }
    }

    pub fn bounds(&self) -> Sphere {
        todo!("Get bounds")
    }

    pub fn filter<N: AsRef<str>>(&self, name: Option<N>) -> Option<MeshFilter> {
        let name_str = deref_str(&name);
        match self
            .meshes
            .binary_search_by(|probe| probe.name().cmp(&name_str))
        {
            Err(_) => None,
            Ok(mut idx) => {
                // Rewind to the start of this same-named group
                while idx > 0 {
                    let next_idx = idx - 1;
                    if self.meshes[next_idx].name() == name_str {
                        idx = next_idx;
                    } else {
                        break;
                    }
                }

                Some(MeshFilter(idx as _))
            }
        }
    }

    pub(crate) fn indices(&self) -> (IndexType, Ref<Lease<Data>>) {
        (self.idx_ty, self.idx_buf.borrow())
    }

    pub(crate) fn indices_mut(&self) -> (IndexType, RefMut<Lease<Data>>) {
        (self.idx_ty, self.idx_buf.borrow_mut())
    }

    pub(super) fn meshes(&self, filter: Option<MeshFilter>) -> MeshIter {
        MeshIter {
            filter,
            idx: 0,
            model: self,
        }
    }

    /// You must submit writes for our buffers if you call this.
    pub(super) fn take_pending_writes(&self) -> Option<(u64, u64)> {
        self.pending_writes.borrow_mut().take()
    }

    pub fn pose_bounds(&self, _pose: &Pose) -> Sphere {
        todo!("Get bounds w/ pose")
    }

    /// Sets a descriptive name for debugging which can be seen with API tracing tools such as RenderDoc.
    #[cfg(debug_assertions)]
    pub fn set_name(&mut self, name: &str) {
        self.idx_buf.borrow_mut().set_name(name);
        self.vertex_buf.borrow_mut().set_name(name);
    }

    pub(crate) fn vertices(&self) -> Ref<Lease<Data>> {
        self.vertex_buf.borrow()
    }

    pub(crate) fn vertices_mut(&self) -> RefMut<Lease<Data>> {
        self.vertex_buf.borrow_mut()
    }
}

impl Debug for Model {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("Model")
    }
}

#[derive(Clone)]
pub struct Pose {
    joints: Vec<Quat>,
}

impl Pose {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            joints: Vec::with_capacity(capacity),
        }
    }

    pub fn joint<N: AsRef<str>>(&self, _name: N) -> Quat {
        // let name = name.as_ref();
        // match self.joints.binary_search_by(|a| name.cmp(&a.0)) {
        //     Err(_) => panic!("Joint not found"),
        //     Ok(idx) => self.joints[idx].1
        // }
        todo!();
    }

    pub fn set_joint(&mut self, _name: String, _val: Quat) {
        // match self.joints.binary_search_by(|a| name.cmp(&a.0)) {
        //     Err(idx) => self.joints.insert(idx, (name, val)),
        //     Ok(idx) => self.joints[idx].1 = val,
        // }
    }
}
