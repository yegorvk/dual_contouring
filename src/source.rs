use auto_impl::auto_impl;
use glam::{vec3, Vec3};

pub struct Sample {
    pub point: Vec3,
    pub value: f32,
}

impl Sample {
    pub fn new(point: Vec3, value: f32) -> Self {
        Self { point, value }
    }

    pub fn from_source(source: impl Source, point: Vec3) -> Self {
        Self::new(point, source.sample(point))
    }
}

pub enum FindIntersectionError {
    IterLimit(Sample),
    Indeterminate,
    NoSolution,
}

pub enum Endpoint {
    Start,
    End,
}

pub enum ClassifySegment {
    ChangesSign(f32, f32),
    Intersects(Endpoint, f32),
    NoSolution,
    Indeterminate,
}

impl ClassifySegment {
    pub fn has_sign_change(&self) -> bool {
        matches!(
            self,
            ClassifySegment::ChangesSign(_, _) | ClassifySegment::Intersects(_, _)
        )
    }
}

#[auto_impl(&, &mut, Box, Rc, Arc)]
pub trait Source {
    /// Samples the source at a given point.
    fn sample(&self, point: Vec3) -> f32;

    fn classify_segment(&self, start: Vec3, end: Vec3, epsilon: f32) -> ClassifySegment {
        debug_assert!(start != end);
        debug_assert!(epsilon > 0.0);

        let v_start = self.sample(start);
        let v_end = self.sample(end);

        // We must consider either `(true, false)` or `(false, true)` as having
        // a solution, but never both simultaneously. This exclusivity is
        // needed to preserve the invariant that one intersection point can
        // only belong to a single segment under an adaptive grid.
        match (v_start.abs() <= epsilon, v_end.abs() <= epsilon) {
            (true, false) => ClassifySegment::Intersects(Endpoint::Start, v_start),
            (true, true) => ClassifySegment::Indeterminate,
            (false, true) => ClassifySegment::NoSolution,
            _ => {
                if v_start.is_sign_negative() != v_end.is_sign_negative() {
                    ClassifySegment::ChangesSign(v_start, v_end)
                } else {
                    ClassifySegment::NoSolution
                }
            }
        }
    }

    fn find_intersection(
        &self,
        start: Vec3,
        end: Vec3,
        epsilon: f32,
        max_iter: usize,
    ) -> Result<Sample, FindIntersectionError> {
        debug_assert!(start != end);
        debug_assert!(epsilon > 0.0);

        match self.classify_segment(start, end, epsilon) {
            ClassifySegment::Intersects(endpoint, value) => {
                return match endpoint {
                    Endpoint::Start => Ok(Sample::new(start, value)),
                    Endpoint::End => Ok(Sample::new(end, value)),
                }
            }
            ClassifySegment::NoSolution => return Err(FindIntersectionError::NoSolution),
            ClassifySegment::Indeterminate => return Err(FindIntersectionError::Indeterminate),
            _ => {}
        }

        let mut a = start;
        let mut b = end;
        let mut v_a = self.sample(a);
        let mut v_b = self.sample(b);

        for _ in 0..max_iter {
            if v_a.is_sign_negative() == v_b.is_sign_negative() {
                return Err(FindIntersectionError::NoSolution);
            }

            if (a - b).length_squared() <= epsilon * epsilon {
                let c = (a + b) / 2.0;
                return Ok(Sample::new(c, self.sample(c)));
            }

            let c = (a + b) / 2.0;
            let v_c = self.sample(c);

            if v_c.abs() <= epsilon {
                return Ok(Sample::new(c, v_c));
            }

            if v_a.is_sign_negative() != v_c.is_sign_negative() {
                b = c;
                v_b = v_c;
            } else {
                a = c;
                v_a = v_c;
            }
        }

        let best = Sample::from_source(self, (a + b) / 2.0);
        Err(FindIntersectionError::IterLimit(best))
    }
}

#[auto_impl(&, &mut, Box, Rc, Arc)]
pub trait HermiteSource: Source {
    fn sample_normal(&self, point: Vec3) -> Vec3;
}

pub struct FiniteDifference<S> {
    source: S,
    epsilon: f32,
}

impl<S> FiniteDifference<S> {
    pub fn new(source: S, epsilon: f32) -> Self {
        Self { source, epsilon }
    }
}

impl<S: Source> Source for FiniteDifference<S> {
    fn sample(&self, point: Vec3) -> f32 {
        self.source.sample(point)
    }
}

impl<S: HermiteSource> HermiteSource for FiniteDifference<S> {
    fn sample_normal(&self, point: Vec3) -> Vec3 {
        let v_x = self.source.sample(point + Vec3::X * self.epsilon);
        let v_y = self.source.sample(point + Vec3::Y * self.epsilon);
        let v_z = self.source.sample(point + Vec3::Z * self.epsilon);
        (vec3(v_x, v_y, v_z) - self.sample(point)).normalize_or_zero()
    }
}
