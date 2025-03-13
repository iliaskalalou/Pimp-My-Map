use crate::obj_export::{
    graph::graph::Graph, line::line::Line, vector3::vector3::Vector3,
};

pub fn divide(
    points: &Vec<usize>,
    nb_x: usize,
    nb_z: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut sections = (Vec::new(), Vec::new());
    let middle = nb_x / 2 + (if nb_x % 2 == 0 { 0 } else { 1 });

    for z in 0..nb_z {
        for x in 0..nb_x {
            if x < middle {
                sections.0.push(points[z * nb_x + x].clone());
            } else {
                sections.1.push(points[z * nb_x + x].clone())
            }
        }
    }
    sections
}

pub fn conquere(
    points: &Vec<usize>,
    nb_x: usize,
    nb_z: usize,
    graph: &mut Graph,
) {
    if points.len() <= nb_z {
        for i in 0..points.len() - 1 {
            graph.add_edges(points[i], points[i + 1]);
        }
        return;
    }

    let (left, right) = divide(points, nb_x, nb_z);
    let new_size_right = right.len() / nb_z; // + (if right.len() % nb_z == 0 {0} else {1});
    let new_size_left = left.len() / nb_z; // + (if left.len() % nb_z == 0 {0} else {1});
    conquere(&left, new_size_left, nb_z, graph);
    conquere(&right, new_size_right, nb_z, graph);
    merge(&left, &right, nb_z, graph)
}

fn find_base_lr(
    src: &Vec<usize>,
    dst: &Vec<usize>,
    nb_z: usize,
    graph: &mut Graph,
) -> (usize, usize) {
    let nb_x = src.len() / nb_z;
    let src_point = src[nb_x - 1];
    let dst_point = dst[0];
    graph.add_edges(src_point, dst_point);
    (src_point, dst_point)
}

fn find_next_candidat(
    base: (usize, usize),
    graph: &mut Graph,
    marq: &Vec<bool>,
) -> Option<(usize, f64)> {
    let line_base = Line::from([
        graph.vertex[base.0].clone(),
        graph.vertex[base.1].clone(),
    ]);
    let mut angles = Vec::new();
    for i in &graph.adjlists[base.1] {
        let line_pot = Line::from([
            graph.vertex[base.1].clone(),
            graph.vertex[*i].clone(),
        ]);
        let angle = line_base.angle(&line_pot);
        if angle < 180.0 && !marq[*i] {
            insert(&mut angles, angle, *i);
        }
    }

    if angles.len() == 0 {
        return None;
    }

    if angles.len() == 1 {
        return Some((angles[0].1, angles[0].0));
    }

    for j in 0..angles.len() - 1 {
        let sommet = [
            graph.vertex[base.0].clone(),
            graph.vertex[base.1].clone(),
            graph.vertex[angles[j].1].clone(),
        ];
        if !in_circumcircle(sommet, &graph.vertex[angles[j + 1].1]) {
            return Some((angles[j].1, angles[j].0));
        }
        graph.remove_edges(base.1, angles[j].1);
    }
    None
}

fn choose_candidat(
    left: Option<(usize, f64)>,
    right: Option<(usize, f64)>,
    base_lr: (usize, usize),
    base_rl: (usize, usize),
    graph: &Graph,
) -> Option<usize> {
    let left = match left {
        Some(x) => x,
        None => (graph.order, 0.0),
    };

    let right = match right {
        Some(x) => x,
        None => (graph.order, 0.0),
    };

    let (first, second) = if left.1 > right.1 {
        (right.0, left.0)
    } else {
        (left.0, right.0)
    };

    if first < graph.order && second < graph.order {
        let sommet = [
            graph.vertex[base_lr.0].clone(),
            graph.vertex[base_lr.1].clone(),
            graph.vertex[first].clone(),
        ];
        if in_circumcircle(sommet, &graph.vertex[second]) {
            let sommet = [
                graph.vertex[base_rl.0].clone(),
                graph.vertex[base_rl.1].clone(),
                graph.vertex[second].clone(),
            ];
            if in_circumcircle(sommet, &graph.vertex[first]) {
                return None;
            }
            return Some(second);
        }
        return Some(first);
    }
    if first < graph.order {
        return Some(first);
    }

    if second < graph.order {
        return Some(second);
    }
    None
}

fn merge(
    left: &Vec<usize>,
    right: &Vec<usize>,
    nb_z: usize,
    graph: &mut Graph,
) {
    let mut base_lr = find_base_lr(left, right, nb_z, graph);
    let mut base_rl = (base_lr.1, base_lr.0);

    let mut marq = vec![false; graph.order];
    marq[base_lr.0] = true;
    marq[base_lr.1] = true;
    let mut pot_right = find_next_candidat(base_lr, graph, &marq);
    let mut pot_left = find_next_candidat(base_rl, graph, &marq);

    let mut candidat =
        choose_candidat(pot_left, pot_right, base_lr, base_rl, graph);
    while candidat.is_some() {
        if graph.add_edges(candidat.unwrap(), base_lr.0) {
            base_lr = (base_lr.0, candidat.unwrap());
        } else {
            base_lr = (candidat.unwrap(), base_lr.1);
        }
        graph.add_edges(candidat.unwrap(), base_lr.1);
        base_rl = (base_lr.1, base_lr.0);

        marq[base_lr.0] = true;
        marq[base_lr.1] = true;

        pot_right = find_next_candidat(base_lr, graph, &marq);
        pot_left = find_next_candidat(base_rl, graph, &marq);
        candidat =
            choose_candidat(pot_left, pot_right, base_lr, base_rl, graph);
    }
}

fn insert(angles: &mut Vec<(f64, usize)>, alpha: f64, i: usize) {
    angles.push((alpha, i));
    let i = angles.len() - 1;
    while i > 0 && angles[i].0 < angles[i - 1].0 {
        let tmp = angles[i];
        angles[i] = angles[i - 1];
        angles[i - 1] = tmp;
    }
}

fn in_circumcircle(sommet: [Vector3; 3], v: &Vector3) -> bool {
    let ax = sommet[0].x - v.x;
    let az = sommet[0].z - v.z;
    let bx = sommet[1].x - v.x;
    let bz = sommet[1].z - v.z;
    let cx = sommet[2].x - v.x;
    let cz = sommet[2].z - v.z;

    let ab =
        ax * (sommet[1].z - sommet[2].z) + az * (sommet[2].x - sommet[1].x);
    let bc =
        bx * (sommet[2].z - sommet[0].z) + bz * (sommet[0].x - sommet[2].x);
    let ca =
        cx * (sommet[0].z - sommet[1].z) + cz * (sommet[1].x - sommet[0].x);
    (ab * ca > 0.0) && (bc * ab > 0.0)
}
