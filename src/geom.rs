use std::mem;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BMask3(u8);

impl BMask3 {
    pub const O: BMask3 = BMask3(0);

    pub const X: BMask3 = BMask3(1 << 0);
    pub const Y: BMask3 = BMask3(1 << 1);
    pub const Z: BMask3 = BMask3(1 << 2);

    pub const XY: BMask3 = BMask3(Self::X.0 | Self::Y.0);
    pub const XZ: BMask3 = BMask3(Self::X.0 | Self::Z.0);
    pub const YZ: BMask3 = BMask3(Self::Y.0 | Self::Z.0);
    pub const XYZ: BMask3 = BMask3(Self::X.0 | Self::Y.0 | Self::Z.0);

    pub const fn bits(&self) -> u8 {
        self.0
    }

    pub const fn step(&self, step: BMask3) -> BMask3 {
        BMask3(self.0 | step.0)
    }

    const fn const_eq(&self, rhs: BMask3) -> bool {
        self.0 == rhs.0
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DirKind {
    X = BMask3::X.bits(), // 1 = 0b001
    Y = BMask3::Y.bits(), // 2 = 0b010
    Z = BMask3::Z.bits(), // 4 = 0b100
}

impl DirKind {
    pub const fn axis(&self) -> AxisKind {
        // SAFETY:
        // - Enums with primitive representation are guaranteed to have
        //   well-defined discriminant values.
        // - The discriminant never has more than 2 trailing zeroes in binary.
        unsafe { mem::transmute((*self as u8).trailing_zeros() as u8) }
    }

    pub const fn to_mask(&self) -> BMask3 {
        BMask3(*self as u8)
    }
}

impl From<DirKind> for BMask3 {
    fn from(value: DirKind) -> Self {
        BMask3(value as u8)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AxisKind {
    X = 0,
    Y = 1,
    Z = 2,
}

impl AxisKind {
    pub const fn faces(&self) -> [FaceKind; 2] {
        let discriminant = *self as u8;
        unsafe { mem::transmute([discriminant << 1, (discriminant << 1) + 1]) }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FaceKind {
    Left = 0,
    Right = 1,
    Bottom = 2,
    Top = 3,
    Back = 4,
    Front = 5,
}

impl FaceKind {
    pub const ALL: [FaceKind; 6] = [
        FaceKind::Left,
        FaceKind::Right,
        FaceKind::Bottom,
        FaceKind::Top,
        FaceKind::Back,
        FaceKind::Front,
    ];

    pub const fn normal_axis(&self) -> AxisKind {
        // SAFETY:
        // - Enums with primitive representation are guaranteed to have
        //   well-defined discriminant values.
        // - `FaceKind` is defined such that discriminants of faces along the
        //   same normal axis are consecutive integers, starting from 0.
        unsafe { mem::transmute((*self as u8) >> 1) }
    }

    pub const fn corners(&self) -> [CornerKind; 4] {
        macro_rules! face_corners {
            ($($face:ident => [$($corner:ident),* $(,)?]),* $(,)?) => {
                match *self {
                    $( FaceKind::$face => [$(CornerKind(BMask3::$corner)),*], )*
                }
            };
        }

        face_corners! {
            Left => [O, Z, YZ, Y],
            Right => [X, XY, XYZ, XZ],
            Bottom => [O, X, XZ, Z],
            Top => [Y, YZ, XYZ, XY],
            Back => [O, Y, XY, X],
            Front => [Z, XZ, XYZ, YZ],
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CornerKind(pub BMask3);

impl CornerKind {
    pub const ALL: [CornerKind; 8] = [
        CornerKind(BMask3::O),
        CornerKind(BMask3::X),
        CornerKind(BMask3::Y),
        CornerKind(BMask3::Z),
        CornerKind(BMask3::XY),
        CornerKind(BMask3::XZ),
        CornerKind(BMask3::YZ),
        CornerKind(BMask3::XYZ),
    ];
}

impl From<CornerKind> for BMask3 {
    fn from(value: CornerKind) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EdgeKind(CornerKind, DirKind);

impl EdgeKind {
    pub const ALL: [EdgeKind; 12] = [
        EdgeKind(CornerKind(BMask3::O), DirKind::X),
        EdgeKind(CornerKind(BMask3::O), DirKind::Y),
        EdgeKind(CornerKind(BMask3::O), DirKind::Z),
        EdgeKind(CornerKind(BMask3::X), DirKind::Y),
        EdgeKind(CornerKind(BMask3::X), DirKind::Z),
        EdgeKind(CornerKind(BMask3::Y), DirKind::X),
        EdgeKind(CornerKind(BMask3::Y), DirKind::Z),
        EdgeKind(CornerKind(BMask3::Z), DirKind::X),
        EdgeKind(CornerKind(BMask3::Z), DirKind::Y),
        EdgeKind(CornerKind(BMask3::XY), DirKind::Z),
        EdgeKind(CornerKind(BMask3::XZ), DirKind::Y),
        EdgeKind(CornerKind(BMask3::XY), DirKind::Z),
    ];

    pub const fn new(start: CornerKind, dir: DirKind) -> Self {
        debug_assert!(!start.0.const_eq(BMask3::XYZ));
        EdgeKind(start, dir)
    }

    pub const fn axis(&self) -> AxisKind {
        self.1.axis()
    }

    pub const fn endpoints(self) -> [CornerKind; 2] {
        let EdgeKind(start, dir) = self;
        [start, CornerKind(start.0.step(dir.to_mask()))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_to_axis() {
        assert_eq!(DirKind::X.axis(), AxisKind::X);
        assert_eq!(DirKind::Y.axis(), AxisKind::Y);
        assert_eq!(DirKind::Z.axis(), AxisKind::Z);
    }

    #[test]
    fn face_normal_axis() {
        assert_eq!(FaceKind::Left.normal_axis(), AxisKind::X);
        assert_eq!(FaceKind::Right.normal_axis(), AxisKind::Y);
        assert_eq!(FaceKind::Bottom.normal_axis(), AxisKind::Y);
        assert_eq!(FaceKind::Top.normal_axis(), AxisKind::Y);
        assert_eq!(FaceKind::Back.normal_axis(), AxisKind::Z);
        assert_eq!(FaceKind::Front.normal_axis(), AxisKind::Z);
    }

    #[test]
    fn axis_faces() {
        assert_eq!(AxisKind::X.faces(), [FaceKind::Left, FaceKind::Right]);
        assert_eq!(AxisKind::Y.faces(), [FaceKind::Bottom, FaceKind::Top]);
        assert_eq!(AxisKind::Z.faces(), [FaceKind::Back, FaceKind::Front]);
    }
}
