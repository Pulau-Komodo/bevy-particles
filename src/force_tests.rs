#[inline]
fn test_force(base_force: f32, proximity_cap: f32, exponent: f32, offset: Vec2) -> Vec2 {
	if offset == Vec2::ZERO {
		return Vec2::ZERO;
	}

	let multiplier = if exponent == 2.0 {
		let dot_product = offset.dot(offset);
		base_force / dot_product.max(proximity_cap * proximity_cap) / dot_product.sqrt()
	} else {
		let squared_dot_product = offset.dot(offset).sqrt();
		base_force / squared_dot_product.max(proximity_cap).powf(exponent) / squared_dot_product
	};

	offset * multiplier
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn force_math() {
		for a in (-10..10).map(|a| a as f32 / 2.0) {
			println!("a: {a}");
			for b in (-10..10).map(|b| b as f32 / 2.0) {
				let offset = Vec2::new(a, b);
				for cap in (0..16).map(|cap| cap as f32) {
					for force in (-8..8).map(|force| force as f32 * 100.0) {
						for exponent in (0..10).map(|exponent| exponent as f32 / 4.0) {
							let value_a = calculate_force(force, cap, exponent, offset);
							let value_b = test_force(force, cap, exponent, offset);
							assert!(
								(value_a.x - value_b.x).abs() < 0.001
									&& (value_a.y - value_b.y).abs() < 0.001
							);
						}
					}
				}
			}
		}
	}
	#[test]
	fn bench_math() {
		let mut list = Vec::with_capacity(20 * 20 * 16 * 16 * 10);
		let timer = std::time::Instant::now();
		for a in (-10..10).map(|a| a as f32 / 2.0) {
			for b in (-10..10).map(|b| b as f32 / 2.0) {
				let offset = Vec2::new(a, b);
				for cap in (0..16).map(|cap| cap as f32) {
					for force in (-8..8).map(|force| force as f32 * 100.0) {
						for exponent in (0..10).map(|exponent| exponent as f32 / 4.0) {
							list.push(calculate_force(force, cap, 2.0, offset));
						}
					}
				}
			}
		}
		println!("Time taken: {:?}", timer.elapsed());
	}
	#[test]
	fn bench_math_2() {
		let mut list = Vec::with_capacity(20 * 20 * 16 * 16 * 10);
		let timer = std::time::Instant::now();
		for a in (-10..10).map(|a| a as f32 / 2.0) {
			for b in (-10..10).map(|b| b as f32 / 2.0) {
				let offset = Vec2::new(a, b);
				for cap in (0..16).map(|cap| cap as f32) {
					for force in (-8..8).map(|force| force as f32 * 100.0) {
						for exponent in (0..10).map(|exponent| exponent as f32 / 4.0) {
							list.push(test_force(force, cap, 2.0, offset));
						}
					}
				}
			}
		}
		println!("Time taken: {:?}", timer.elapsed());
	}
}
