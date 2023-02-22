//! Component used in the ALU.

use crate::add;
use crate::bus::BusLayoutCommand;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;
use crate::utils::get_pin_coords;

pub fn get_zero_tester_circuit() -> Circuit {
	let width = 3000.0;
	let layer_xs = get_pin_coords(0.0, 4, 500.0);

	let mut circuit = Circuit::new();

	let input: Vec<_> = get_pin_coords(0.0, 16, 50.0).iter()
		.map(|y| add!(circuit, Pin, (-width * 0.5, *y)))
		.collect();

	let output = add!(circuit, Pin, (width * 0.5, 0.0));

	let layer1: Vec<_> = get_pin_coords(0.0, 8, 300.0).iter()
		.map(|y| add!(circuit, NorGate, (layer_xs[0], *y)))
		.collect();

	let layer2: Vec<_> = get_pin_coords(0.0, 4, 300.0).iter()
		.map(|y| add!(circuit, NandGate, (layer_xs[1], *y)))
		.collect();

	let layer3: Vec<_> = get_pin_coords(0.0, 2, 300.0).iter()
		.map(|y| add!(circuit, NorGate, (layer_xs[2], *y)))
		.collect();

	let layer4: Vec<_> = get_pin_coords(0.0, 1, 300.0).iter()
		.map(|y| add!(circuit, AndGate, (layer_xs[3], *y)))
		.collect();
	
	circuit.connect_groups(
		&input[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&layer1[..4].iter().flat_map(|c| [(*c, 0), (*c, 1)]).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&input[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&layer1[4..].iter().flat_map(|c| [(*c, 0), (*c, 1)]).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	let wire_coords = get_pin_coords((layer_xs[0] + layer_xs[1]) * 0.5, 4, 40.0);

	for i in 0..4 {
		circuit.connect((layer1[i], 2), (layer2[i/2], i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[3-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((layer1[7-i], 2), (layer2[3-i/2], 1 - i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[3-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
	}
	
	let wire_coords = get_pin_coords((layer_xs[1] + layer_xs[2]) * 0.5, 2, 40.0);

	for i in 0..2 {
		circuit.connect((layer2[i], 2), (layer3[i/2], i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[1-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
		circuit.connect((layer2[3-i], 2), (layer3[1-i/2], 1 - i % 2), &[
			WireLayoutCommand::MoveXTo(wire_coords[1-i]),
			WireLayoutCommand::AlignHorizontal,
		]);
	}

	circuit.connect((layer3[0], 2), (layer4[0], 0), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((layer3[1], 2), (layer4[0], 1), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((layer4[0], 2), (output, 0), &[]);

	circuit
}

pub fn get_conditional_inverter_circuit(size: usize) -> Circuit {
	let mut circuit = Circuit::new();

	let input: Vec<_> = get_pin_coords(0.0, size, 50.0).iter()
		.map(|y| add!(circuit, Pin, (-1500.0, *y)))
		.collect();

	let enable = add!(circuit, Pin, (0.0, 2500.0));

	let xors: Vec<_> = get_pin_coords(0.0, size, 250.0).iter()
		.map(|y| add!(circuit, XorGate, (0.0, *y)))
		.collect();

	let junctions: Vec<_> = xors[1..].iter()
		.map(|c| {
			let pos = (
				circuit.components[*c].position.0 + circuit.components[*c].get_pin_positions()[1].0,
				circuit.components[*c].position.1 + circuit.components[*c].get_pin_positions()[1].1,
			);
			add!(circuit, Junction, (pos.0 - 150.0, pos.1), 3)
		})
		.collect();

	let output: Vec<_> = get_pin_coords(0.0, size, 50.0).iter()
		.map(|y| add!(circuit, Pin, (1500.0, *y)))
		.collect();

	circuit.connect_groups(
		&input[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&input[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	for i in 0..size-1 {
		circuit.connect((junctions[i], 2), (xors[i+1], 1), &[]);
	}
	for i in 0..size-2 {
		circuit.connect((junctions[i], 1), (junctions[i+1], 0), &[]);
	}

	circuit.connect((enable, 0), (junctions[size-2], 1), &[
		WireLayoutCommand::CenterVertical,
		WireLayoutCommand::AlignVertical,
	]);
	circuit.connect((junctions[0], 0), (xors[0], 1), &[
		WireLayoutCommand::AlignHorizontal,
	]);
	
	circuit.connect_groups(
		&output[..8].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[..8].iter().map(|c| (*c, 2)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&output[8..].iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&xors[8..].iter().map(|c| (*c, 2)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit
}

pub fn get_alu_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, Adder, (0.000, 0.000), 16);
	let c1 = add!(circuit, ConditionalInverter, (-1100.000, 600.000), 16);
	let c2 = add!(circuit, MultiSwitch, (-850.000, 1800.000), 16);
	let c3 = add!(circuit, MultiSwitch, (850.000, 1800.000), 16);
	let c4 = add!(circuit, Switch, (-2500.000, 0.000));
	let c5 = add!(circuit, Junction, (-1100.000, 1150.000), 3);
	let c6 = add!(circuit, ZeroTester, (2000.000, 0.000));
	let c7 = add!(circuit, MultiJunction, (1050.000, 0.000), 16);
	let c8 = add!(circuit, MultiBulb, (-50.000, -1800.000), 16);
	let c9 = add!(circuit, Bulb, (2500.000, 0.000));
	
	circuit.connect((c1, 17), (c0, 31), &[WireLayoutCommand::MoveTo((-600.000, 525.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-75.000, -0.000)), WireLayoutCommand::MoveTo((-675.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 18), (c0, 30), &[WireLayoutCommand::MoveTo((-600.000, 535.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-65.000, -0.000)), WireLayoutCommand::MoveTo((-665.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 19), (c0, 29), &[WireLayoutCommand::MoveTo((-600.000, 545.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-55.000, -0.000)), WireLayoutCommand::MoveTo((-655.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 20), (c0, 28), &[WireLayoutCommand::MoveTo((-600.000, 555.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-45.000, -0.000)), WireLayoutCommand::MoveTo((-645.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 21), (c0, 27), &[WireLayoutCommand::MoveTo((-600.000, 565.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-35.000, -0.000)), WireLayoutCommand::MoveTo((-635.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 22), (c0, 26), &[WireLayoutCommand::MoveTo((-600.000, 575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.000, -0.000)), WireLayoutCommand::MoveTo((-625.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 23), (c0, 25), &[WireLayoutCommand::MoveTo((-600.000, 585.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, -0.000)), WireLayoutCommand::MoveTo((-615.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 24), (c0, 24), &[WireLayoutCommand::MoveTo((-600.000, 595.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, -0.000)), WireLayoutCommand::MoveTo((-605.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 25), (c0, 23), &[WireLayoutCommand::MoveTo((-600.000, 605.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, 0.000)), WireLayoutCommand::MoveTo((-595.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 26), (c0, 22), &[WireLayoutCommand::MoveTo((-600.000, 615.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, 0.000)), WireLayoutCommand::MoveTo((-585.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 27), (c0, 21), &[WireLayoutCommand::MoveTo((-600.000, 625.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.000, 0.000)), WireLayoutCommand::MoveTo((-575.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 28), (c0, 20), &[WireLayoutCommand::MoveTo((-600.000, 635.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((35.000, 0.000)), WireLayoutCommand::MoveTo((-565.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 29), (c0, 19), &[WireLayoutCommand::MoveTo((-600.000, 645.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((45.000, 0.000)), WireLayoutCommand::MoveTo((-555.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 30), (c0, 18), &[WireLayoutCommand::MoveTo((-600.000, 655.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((55.000, 0.000)), WireLayoutCommand::MoveTo((-545.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 31), (c0, 17), &[WireLayoutCommand::MoveTo((-600.000, 665.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((65.000, 0.000)), WireLayoutCommand::MoveTo((-535.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 32), (c0, 16), &[WireLayoutCommand::MoveTo((-600.000, 675.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((75.000, 0.000)), WireLayoutCommand::MoveTo((-525.000, 307.500)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 0), (c3, 0), &[WireLayoutCommand::MoveTo((-1650.000, 525.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-75.000, 0.000)), WireLayoutCommand::MoveTo((-1725.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 75.000)), WireLayoutCommand::MoveTo((850.000, 1325.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 1), (c3, 1), &[WireLayoutCommand::MoveTo((-1650.000, 535.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-65.000, 0.000)), WireLayoutCommand::MoveTo((-1715.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 65.000)), WireLayoutCommand::MoveTo((850.000, 1315.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 2), (c3, 2), &[WireLayoutCommand::MoveTo((-1650.000, 545.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-55.000, 0.000)), WireLayoutCommand::MoveTo((-1705.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 55.000)), WireLayoutCommand::MoveTo((850.000, 1305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 3), (c3, 3), &[WireLayoutCommand::MoveTo((-1650.000, 555.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-45.000, 0.000)), WireLayoutCommand::MoveTo((-1695.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 45.000)), WireLayoutCommand::MoveTo((850.000, 1295.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 4), (c3, 4), &[WireLayoutCommand::MoveTo((-1650.000, 565.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-35.000, 0.000)), WireLayoutCommand::MoveTo((-1685.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 35.000)), WireLayoutCommand::MoveTo((850.000, 1285.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 5), (c3, 5), &[WireLayoutCommand::MoveTo((-1650.000, 575.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.000, 0.000)), WireLayoutCommand::MoveTo((-1675.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 25.000)), WireLayoutCommand::MoveTo((850.000, 1275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 6), (c3, 6), &[WireLayoutCommand::MoveTo((-1650.000, 585.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, 0.000)), WireLayoutCommand::MoveTo((-1665.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((850.000, 1265.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 7), (c3, 7), &[WireLayoutCommand::MoveTo((-1650.000, 595.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, 0.000)), WireLayoutCommand::MoveTo((-1655.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 5.000)), WireLayoutCommand::MoveTo((850.000, 1255.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 8), (c3, 8), &[WireLayoutCommand::MoveTo((-1650.000, 605.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, -0.000)), WireLayoutCommand::MoveTo((-1645.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -5.000)), WireLayoutCommand::MoveTo((850.000, 1245.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 9), (c3, 9), &[WireLayoutCommand::MoveTo((-1650.000, 615.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, -0.000)), WireLayoutCommand::MoveTo((-1635.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((850.000, 1235.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 10), (c3, 10), &[WireLayoutCommand::MoveTo((-1650.000, 625.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.000, -0.000)), WireLayoutCommand::MoveTo((-1625.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -25.000)), WireLayoutCommand::MoveTo((850.000, 1225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 11), (c3, 11), &[WireLayoutCommand::MoveTo((-1650.000, 635.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((35.000, -0.000)), WireLayoutCommand::MoveTo((-1615.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -35.000)), WireLayoutCommand::MoveTo((850.000, 1215.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 12), (c3, 12), &[WireLayoutCommand::MoveTo((-1650.000, 645.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((45.000, -0.000)), WireLayoutCommand::MoveTo((-1605.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -45.000)), WireLayoutCommand::MoveTo((850.000, 1205.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 13), (c3, 13), &[WireLayoutCommand::MoveTo((-1650.000, 655.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((55.000, -0.000)), WireLayoutCommand::MoveTo((-1595.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -55.000)), WireLayoutCommand::MoveTo((850.000, 1195.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 14), (c3, 14), &[WireLayoutCommand::MoveTo((-1650.000, 665.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((65.000, -0.000)), WireLayoutCommand::MoveTo((-1585.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -65.000)), WireLayoutCommand::MoveTo((850.000, 1185.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 15), (c3, 15), &[WireLayoutCommand::MoveTo((-1650.000, 675.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((75.000, -0.000)), WireLayoutCommand::MoveTo((-1575.000, 1250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -75.000)), WireLayoutCommand::MoveTo((850.000, 1175.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 15), (c2, 0), &[WireLayoutCommand::MoveTo((-1900.000, -405.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, 0.000)), WireLayoutCommand::MoveTo((-2012.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 112.500)), WireLayoutCommand::MoveTo((-850.000, 1612.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 14), (c2, 1), &[WireLayoutCommand::MoveTo((-1900.000, -390.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, 0.000)), WireLayoutCommand::MoveTo((-1997.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 97.500)), WireLayoutCommand::MoveTo((-850.000, 1597.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 13), (c2, 2), &[WireLayoutCommand::MoveTo((-1900.000, -375.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, 0.000)), WireLayoutCommand::MoveTo((-1982.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 82.500)), WireLayoutCommand::MoveTo((-850.000, 1582.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 12), (c2, 3), &[WireLayoutCommand::MoveTo((-1900.000, -360.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, 0.000)), WireLayoutCommand::MoveTo((-1967.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 67.500)), WireLayoutCommand::MoveTo((-850.000, 1567.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 11), (c2, 4), &[WireLayoutCommand::MoveTo((-1900.000, -345.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((-1952.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 52.500)), WireLayoutCommand::MoveTo((-850.000, 1552.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 10), (c2, 5), &[WireLayoutCommand::MoveTo((-1900.000, -330.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((-1937.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 37.500)), WireLayoutCommand::MoveTo((-850.000, 1537.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 9), (c2, 6), &[WireLayoutCommand::MoveTo((-1900.000, -315.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((-1922.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 22.500)), WireLayoutCommand::MoveTo((-850.000, 1522.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 8), (c2, 7), &[WireLayoutCommand::MoveTo((-1900.000, -300.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((-1907.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 7.500)), WireLayoutCommand::MoveTo((-850.000, 1507.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 7), (c2, 8), &[WireLayoutCommand::MoveTo((-1900.000, -285.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((-1892.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -7.500)), WireLayoutCommand::MoveTo((-850.000, 1492.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 6), (c2, 9), &[WireLayoutCommand::MoveTo((-1900.000, -270.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((-1877.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -22.500)), WireLayoutCommand::MoveTo((-850.000, 1477.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 5), (c2, 10), &[WireLayoutCommand::MoveTo((-1900.000, -255.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((-1862.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -37.500)), WireLayoutCommand::MoveTo((-850.000, 1462.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 4), (c2, 11), &[WireLayoutCommand::MoveTo((-1900.000, -240.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((-1847.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -52.500)), WireLayoutCommand::MoveTo((-850.000, 1447.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 3), (c2, 12), &[WireLayoutCommand::MoveTo((-1900.000, -225.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, -0.000)), WireLayoutCommand::MoveTo((-1832.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -67.500)), WireLayoutCommand::MoveTo((-850.000, 1432.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 2), (c2, 13), &[WireLayoutCommand::MoveTo((-1900.000, -210.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, -0.000)), WireLayoutCommand::MoveTo((-1817.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -82.500)), WireLayoutCommand::MoveTo((-850.000, 1417.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 1), (c2, 14), &[WireLayoutCommand::MoveTo((-1900.000, -195.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, -0.000)), WireLayoutCommand::MoveTo((-1802.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -97.500)), WireLayoutCommand::MoveTo((-850.000, 1402.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 0), (c2, 15), &[WireLayoutCommand::MoveTo((-1900.000, -180.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, -0.000)), WireLayoutCommand::MoveTo((-1787.500, 1500.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -112.500)), WireLayoutCommand::MoveTo((-850.000, 1387.500)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c4, 0), (c5, 0), &[WireLayoutCommand::MoveTo((-1500.000, 0.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1500.000, 1150.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 1), (c1, 16), &[]);
	circuit.connect((c5, 2), (c0, 48), &[WireLayoutCommand::MoveTo((0.000, 1150.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c0, 47), (c7, 0), &[WireLayoutCommand::MoveTo((550.000, -112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((497.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 46), (c7, 3), &[WireLayoutCommand::MoveTo((550.000, -97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((512.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 45), (c7, 6), &[WireLayoutCommand::MoveTo((550.000, -82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((527.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 44), (c7, 9), &[WireLayoutCommand::MoveTo((550.000, -67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((542.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 43), (c7, 12), &[WireLayoutCommand::MoveTo((550.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((557.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 42), (c7, 15), &[WireLayoutCommand::MoveTo((550.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((572.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 41), (c7, 18), &[WireLayoutCommand::MoveTo((550.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((587.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 40), (c7, 21), &[WireLayoutCommand::MoveTo((550.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((602.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 39), (c7, 24), &[WireLayoutCommand::MoveTo((550.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((602.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 38), (c7, 27), &[WireLayoutCommand::MoveTo((550.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((587.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 37), (c7, 30), &[WireLayoutCommand::MoveTo((550.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((572.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 36), (c7, 33), &[WireLayoutCommand::MoveTo((550.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((557.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 35), (c7, 36), &[WireLayoutCommand::MoveTo((550.000, 67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((542.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 34), (c7, 39), &[WireLayoutCommand::MoveTo((550.000, 82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((527.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 33), (c7, 42), &[WireLayoutCommand::MoveTo((550.000, 97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((512.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 32), (c7, 45), &[WireLayoutCommand::MoveTo((550.000, 112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((497.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c7, 1), (c8, 0), &[WireLayoutCommand::MoveTo((825.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 225.000)), WireLayoutCommand::MoveTo((-50.000, -975.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 4), (c8, 1), &[WireLayoutCommand::MoveTo((855.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 195.000)), WireLayoutCommand::MoveTo((-50.000, -1005.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 7), (c8, 2), &[WireLayoutCommand::MoveTo((885.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 165.000)), WireLayoutCommand::MoveTo((-50.000, -1035.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 10), (c8, 3), &[WireLayoutCommand::MoveTo((915.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 135.000)), WireLayoutCommand::MoveTo((-50.000, -1065.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 13), (c8, 4), &[WireLayoutCommand::MoveTo((945.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 105.000)), WireLayoutCommand::MoveTo((-50.000, -1095.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 16), (c8, 5), &[WireLayoutCommand::MoveTo((975.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((-50.000, -1125.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 19), (c8, 6), &[WireLayoutCommand::MoveTo((1005.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((-50.000, -1155.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 22), (c8, 7), &[WireLayoutCommand::MoveTo((1035.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-50.000, -1185.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 25), (c8, 8), &[WireLayoutCommand::MoveTo((1065.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-50.000, -1215.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 28), (c8, 9), &[WireLayoutCommand::MoveTo((1095.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((-50.000, -1245.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 31), (c8, 10), &[WireLayoutCommand::MoveTo((1125.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((-50.000, -1275.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 34), (c8, 11), &[WireLayoutCommand::MoveTo((1155.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -105.000)), WireLayoutCommand::MoveTo((-50.000, -1305.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 37), (c8, 12), &[WireLayoutCommand::MoveTo((1185.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -135.000)), WireLayoutCommand::MoveTo((-50.000, -1335.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 40), (c8, 13), &[WireLayoutCommand::MoveTo((1215.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -165.000)), WireLayoutCommand::MoveTo((-50.000, -1365.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 43), (c8, 14), &[WireLayoutCommand::MoveTo((1245.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -195.000)), WireLayoutCommand::MoveTo((-50.000, -1395.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 46), (c8, 15), &[WireLayoutCommand::MoveTo((1275.000, -1200.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -225.000)), WireLayoutCommand::MoveTo((-50.000, -1425.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c6, 0), (c7, 2), &[WireLayoutCommand::MoveTo((1550.000, -75.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((35.000, -0.000)), WireLayoutCommand::MoveTo((1585.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 1), (c7, 5), &[WireLayoutCommand::MoveTo((1550.000, -65.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.000, -0.000)), WireLayoutCommand::MoveTo((1575.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 2), (c7, 8), &[WireLayoutCommand::MoveTo((1550.000, -55.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, -0.000)), WireLayoutCommand::MoveTo((1565.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 3), (c7, 11), &[WireLayoutCommand::MoveTo((1550.000, -45.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, -0.000)), WireLayoutCommand::MoveTo((1555.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 4), (c7, 14), &[WireLayoutCommand::MoveTo((1550.000, -35.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, 0.000)), WireLayoutCommand::MoveTo((1545.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 5), (c7, 17), &[WireLayoutCommand::MoveTo((1550.000, -25.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, 0.000)), WireLayoutCommand::MoveTo((1535.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 6), (c7, 20), &[WireLayoutCommand::MoveTo((1550.000, -15.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.000, 0.000)), WireLayoutCommand::MoveTo((1525.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 7), (c7, 23), &[WireLayoutCommand::MoveTo((1550.000, -5.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-35.000, 0.000)), WireLayoutCommand::MoveTo((1515.000, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 8), (c7, 26), &[WireLayoutCommand::MoveTo((1550.000, 5.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-35.000, 0.000)), WireLayoutCommand::MoveTo((1515.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 9), (c7, 29), &[WireLayoutCommand::MoveTo((1550.000, 15.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.000, 0.000)), WireLayoutCommand::MoveTo((1525.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 10), (c7, 32), &[WireLayoutCommand::MoveTo((1550.000, 25.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-15.000, 0.000)), WireLayoutCommand::MoveTo((1535.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 11), (c7, 35), &[WireLayoutCommand::MoveTo((1550.000, 35.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-5.000, 0.000)), WireLayoutCommand::MoveTo((1545.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 12), (c7, 38), &[WireLayoutCommand::MoveTo((1550.000, 45.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((5.000, -0.000)), WireLayoutCommand::MoveTo((1555.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 13), (c7, 41), &[WireLayoutCommand::MoveTo((1550.000, 55.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((15.000, -0.000)), WireLayoutCommand::MoveTo((1565.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 14), (c7, 44), &[WireLayoutCommand::MoveTo((1550.000, 65.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.000, -0.000)), WireLayoutCommand::MoveTo((1575.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 15), (c7, 47), &[WireLayoutCommand::MoveTo((1550.000, 75.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((35.000, -0.000)), WireLayoutCommand::MoveTo((1585.000, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c6, 16), (c9, 0), &[]);
	
	circuit.pinify(&mut [c2, c3, c4, c8, c9]);

	circuit
}
