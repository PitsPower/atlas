use crate::add;
use crate::bus::BusLayoutCommand;
use crate::core::{Circuit, ComponentOptions, ComponentType};
use crate::graphics::WireLayoutCommand;
use crate::utils::get_pin_coords;

pub fn get_register_circuit() -> Circuit {
	let mut circuit = Circuit::new();

	let c0 = add!(circuit, MultiTriStateBuffer, (650.000, 0.000), 16);
	let c1 = add!(circuit, MultiDFlipFlop, (-650.000, 0.000), 16);
	let c2 = add!(circuit, Switch, (-1750.000, 0.000));
	let c3 = add!(circuit, Switch, (-1750.000, 250.000));
	let c4 = add!(circuit, MultiSwitch, (0.000, 2000.000), 16);
	let c5 = add!(circuit, MultiJunction, (0.000, 0.000), 16);
	let c6 = add!(circuit, MultiBulb, (0.000, -2000.000), 16);
	let c7 = add!(circuit, MultiJunction, (0.000, 1300.000), 16, true);
	let c8 = add!(circuit, AndGate, (-1550.000, 150.000));
	let c9 = add!(circuit, Switch, (-1750.000, -250.000));
	
	circuit.connect((c1, 32), (c5, 0), &[WireLayoutCommand::MoveTo((-350.000, -128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-59.850, -0.000)), WireLayoutCommand::MoveTo((-409.850, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 31), (c5, 3), &[WireLayoutCommand::MoveTo((-350.000, -111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-42.750, -0.000)), WireLayoutCommand::MoveTo((-392.750, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 30), (c5, 6), &[WireLayoutCommand::MoveTo((-350.000, -94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.650, -0.000)), WireLayoutCommand::MoveTo((-375.650, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 29), (c5, 9), &[WireLayoutCommand::MoveTo((-350.000, -76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.550, -0.000)), WireLayoutCommand::MoveTo((-358.550, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 28), (c5, 12), &[WireLayoutCommand::MoveTo((-350.000, -59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.550, 0.000)), WireLayoutCommand::MoveTo((-341.450, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 27), (c5, 15), &[WireLayoutCommand::MoveTo((-350.000, -42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.650, 0.000)), WireLayoutCommand::MoveTo((-324.350, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 26), (c5, 18), &[WireLayoutCommand::MoveTo((-350.000, -25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((42.750, 0.000)), WireLayoutCommand::MoveTo((-307.250, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 25), (c5, 21), &[WireLayoutCommand::MoveTo((-350.000, -8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.850, 0.000)), WireLayoutCommand::MoveTo((-290.150, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 24), (c5, 24), &[WireLayoutCommand::MoveTo((-350.000, 8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.850, 0.000)), WireLayoutCommand::MoveTo((-290.150, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 23), (c5, 27), &[WireLayoutCommand::MoveTo((-350.000, 25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((42.750, 0.000)), WireLayoutCommand::MoveTo((-307.250, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 22), (c5, 30), &[WireLayoutCommand::MoveTo((-350.000, 42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.650, 0.000)), WireLayoutCommand::MoveTo((-324.350, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 21), (c5, 33), &[WireLayoutCommand::MoveTo((-350.000, 59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.550, 0.000)), WireLayoutCommand::MoveTo((-341.450, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 20), (c5, 36), &[WireLayoutCommand::MoveTo((-350.000, 76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.550, -0.000)), WireLayoutCommand::MoveTo((-358.550, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 19), (c5, 39), &[WireLayoutCommand::MoveTo((-350.000, 94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.650, -0.000)), WireLayoutCommand::MoveTo((-375.650, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 18), (c5, 42), &[WireLayoutCommand::MoveTo((-350.000, 111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-42.750, -0.000)), WireLayoutCommand::MoveTo((-392.750, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 17), (c5, 45), &[WireLayoutCommand::MoveTo((-350.000, 128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-59.850, -0.000)), WireLayoutCommand::MoveTo((-409.850, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 0), (c5, 1), &[WireLayoutCommand::MoveTo((400.000, -112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((452.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 1), (c5, 4), &[WireLayoutCommand::MoveTo((400.000, -97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((437.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 2), (c5, 7), &[WireLayoutCommand::MoveTo((400.000, -82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((422.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 3), (c5, 10), &[WireLayoutCommand::MoveTo((400.000, -67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((407.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 4), (c5, 13), &[WireLayoutCommand::MoveTo((400.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((392.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 5), (c5, 16), &[WireLayoutCommand::MoveTo((400.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((377.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 6), (c5, 19), &[WireLayoutCommand::MoveTo((400.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((362.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 7), (c5, 22), &[WireLayoutCommand::MoveTo((400.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((347.500, -120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 8), (c5, 25), &[WireLayoutCommand::MoveTo((400.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, 0.000)), WireLayoutCommand::MoveTo((347.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 9), (c5, 28), &[WireLayoutCommand::MoveTo((400.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, 0.000)), WireLayoutCommand::MoveTo((362.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 10), (c5, 31), &[WireLayoutCommand::MoveTo((400.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, 0.000)), WireLayoutCommand::MoveTo((377.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 11), (c5, 34), &[WireLayoutCommand::MoveTo((400.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, 0.000)), WireLayoutCommand::MoveTo((392.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 12), (c5, 37), &[WireLayoutCommand::MoveTo((400.000, 67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, -0.000)), WireLayoutCommand::MoveTo((407.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 13), (c5, 40), &[WireLayoutCommand::MoveTo((400.000, 82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, -0.000)), WireLayoutCommand::MoveTo((422.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 14), (c5, 43), &[WireLayoutCommand::MoveTo((400.000, 97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, -0.000)), WireLayoutCommand::MoveTo((437.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 15), (c5, 46), &[WireLayoutCommand::MoveTo((400.000, 112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, -0.000)), WireLayoutCommand::MoveTo((452.500, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c5, 2), (c6, 0), &[WireLayoutCommand::MoveTo((-225.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 105.000)), WireLayoutCommand::MoveTo((-200.000, -1045.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 5), (c6, 1), &[WireLayoutCommand::MoveTo((-195.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((-200.000, -1075.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 8), (c6, 2), &[WireLayoutCommand::MoveTo((-165.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((-200.000, -1105.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 11), (c6, 3), &[WireLayoutCommand::MoveTo((-135.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((-200.000, -1135.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 14), (c6, 4), &[WireLayoutCommand::MoveTo((-105.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((-200.000, -1165.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 17), (c6, 5), &[WireLayoutCommand::MoveTo((-75.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((-200.000, -1195.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 20), (c6, 6), &[WireLayoutCommand::MoveTo((-45.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((-200.000, -1225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 23), (c6, 7), &[WireLayoutCommand::MoveTo((-15.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -105.000)), WireLayoutCommand::MoveTo((-200.000, -1255.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 26), (c6, 8), &[WireLayoutCommand::MoveTo((15.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -105.000)), WireLayoutCommand::MoveTo((200.000, -1255.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 29), (c6, 9), &[WireLayoutCommand::MoveTo((45.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -75.000)), WireLayoutCommand::MoveTo((200.000, -1225.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 32), (c6, 10), &[WireLayoutCommand::MoveTo((75.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -45.000)), WireLayoutCommand::MoveTo((200.000, -1195.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 35), (c6, 11), &[WireLayoutCommand::MoveTo((105.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -15.000)), WireLayoutCommand::MoveTo((200.000, -1165.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 38), (c6, 12), &[WireLayoutCommand::MoveTo((135.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 15.000)), WireLayoutCommand::MoveTo((200.000, -1135.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 41), (c6, 13), &[WireLayoutCommand::MoveTo((165.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 45.000)), WireLayoutCommand::MoveTo((200.000, -1105.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 44), (c6, 14), &[WireLayoutCommand::MoveTo((195.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 75.000)), WireLayoutCommand::MoveTo((200.000, -1075.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c5, 47), (c6, 15), &[WireLayoutCommand::MoveTo((225.000, -1150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, 105.000)), WireLayoutCommand::MoveTo((200.000, -1045.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c1, 15), (c7, 0), &[WireLayoutCommand::MoveTo((-1100.000, -128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-128.250, 0.000)), WireLayoutCommand::MoveTo((-1228.250, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 14), (c7, 3), &[WireLayoutCommand::MoveTo((-1100.000, -111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-111.150, 0.000)), WireLayoutCommand::MoveTo((-1211.150, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 13), (c7, 6), &[WireLayoutCommand::MoveTo((-1100.000, -94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-94.050, 0.000)), WireLayoutCommand::MoveTo((-1194.050, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 12), (c7, 9), &[WireLayoutCommand::MoveTo((-1100.000, -76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-76.950, 0.000)), WireLayoutCommand::MoveTo((-1176.950, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 11), (c7, 12), &[WireLayoutCommand::MoveTo((-1100.000, -59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-59.850, 0.000)), WireLayoutCommand::MoveTo((-1159.850, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 10), (c7, 15), &[WireLayoutCommand::MoveTo((-1100.000, -42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-42.750, 0.000)), WireLayoutCommand::MoveTo((-1142.750, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 9), (c7, 18), &[WireLayoutCommand::MoveTo((-1100.000, -25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-25.650, 0.000)), WireLayoutCommand::MoveTo((-1125.650, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 8), (c7, 21), &[WireLayoutCommand::MoveTo((-1100.000, -8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-8.550, 0.000)), WireLayoutCommand::MoveTo((-1108.550, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 7), (c7, 24), &[WireLayoutCommand::MoveTo((-1100.000, 8.550)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((8.550, -0.000)), WireLayoutCommand::MoveTo((-1091.450, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 6), (c7, 27), &[WireLayoutCommand::MoveTo((-1100.000, 25.650)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((25.650, -0.000)), WireLayoutCommand::MoveTo((-1074.350, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 5), (c7, 30), &[WireLayoutCommand::MoveTo((-1100.000, 42.750)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((42.750, -0.000)), WireLayoutCommand::MoveTo((-1057.250, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 4), (c7, 33), &[WireLayoutCommand::MoveTo((-1100.000, 59.850)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((59.850, -0.000)), WireLayoutCommand::MoveTo((-1040.150, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 3), (c7, 36), &[WireLayoutCommand::MoveTo((-1100.000, 76.950)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((76.950, -0.000)), WireLayoutCommand::MoveTo((-1023.050, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 2), (c7, 39), &[WireLayoutCommand::MoveTo((-1100.000, 94.050)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((94.050, -0.000)), WireLayoutCommand::MoveTo((-1005.950, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 1), (c7, 42), &[WireLayoutCommand::MoveTo((-1100.000, 111.150)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((111.150, -0.000)), WireLayoutCommand::MoveTo((-988.850, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c1, 0), (c7, 45), &[WireLayoutCommand::MoveTo((-1100.000, 128.250)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((128.250, -0.000)), WireLayoutCommand::MoveTo((-971.750, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 17), (c7, 1), &[WireLayoutCommand::MoveTo((1050.000, -112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((112.500, 0.000)), WireLayoutCommand::MoveTo((1162.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 18), (c7, 4), &[WireLayoutCommand::MoveTo((1050.000, -97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((97.500, 0.000)), WireLayoutCommand::MoveTo((1147.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 19), (c7, 7), &[WireLayoutCommand::MoveTo((1050.000, -82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((82.500, 0.000)), WireLayoutCommand::MoveTo((1132.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 20), (c7, 10), &[WireLayoutCommand::MoveTo((1050.000, -67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((67.500, 0.000)), WireLayoutCommand::MoveTo((1117.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 21), (c7, 13), &[WireLayoutCommand::MoveTo((1050.000, -52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((52.500, 0.000)), WireLayoutCommand::MoveTo((1102.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 22), (c7, 16), &[WireLayoutCommand::MoveTo((1050.000, -37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((37.500, 0.000)), WireLayoutCommand::MoveTo((1087.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 23), (c7, 19), &[WireLayoutCommand::MoveTo((1050.000, -22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((22.500, 0.000)), WireLayoutCommand::MoveTo((1072.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 24), (c7, 22), &[WireLayoutCommand::MoveTo((1050.000, -7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((7.500, 0.000)), WireLayoutCommand::MoveTo((1057.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 25), (c7, 25), &[WireLayoutCommand::MoveTo((1050.000, 7.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-7.500, -0.000)), WireLayoutCommand::MoveTo((1042.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 26), (c7, 28), &[WireLayoutCommand::MoveTo((1050.000, 22.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-22.500, -0.000)), WireLayoutCommand::MoveTo((1027.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 27), (c7, 31), &[WireLayoutCommand::MoveTo((1050.000, 37.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-37.500, -0.000)), WireLayoutCommand::MoveTo((1012.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 28), (c7, 34), &[WireLayoutCommand::MoveTo((1050.000, 52.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-52.500, -0.000)), WireLayoutCommand::MoveTo((997.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 29), (c7, 37), &[WireLayoutCommand::MoveTo((1050.000, 67.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-67.500, -0.000)), WireLayoutCommand::MoveTo((982.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 30), (c7, 40), &[WireLayoutCommand::MoveTo((1050.000, 82.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-82.500, -0.000)), WireLayoutCommand::MoveTo((967.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 31), (c7, 43), &[WireLayoutCommand::MoveTo((1050.000, 97.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-97.500, -0.000)), WireLayoutCommand::MoveTo((952.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c0, 32), (c7, 46), &[WireLayoutCommand::MoveTo((1050.000, 112.500)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-112.500, -0.000)), WireLayoutCommand::MoveTo((937.500, 1300.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c7, 2), (c4, 0), &[WireLayoutCommand::MoveTo((-225.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -105.000)), WireLayoutCommand::MoveTo((-200.000, 1595.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 5), (c4, 1), &[WireLayoutCommand::MoveTo((-195.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -75.000)), WireLayoutCommand::MoveTo((-200.000, 1625.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 8), (c4, 2), &[WireLayoutCommand::MoveTo((-165.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -45.000)), WireLayoutCommand::MoveTo((-200.000, 1655.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 11), (c4, 3), &[WireLayoutCommand::MoveTo((-135.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((-200.000, 1685.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 14), (c4, 4), &[WireLayoutCommand::MoveTo((-105.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((-200.000, 1715.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 17), (c4, 5), &[WireLayoutCommand::MoveTo((-75.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 45.000)), WireLayoutCommand::MoveTo((-200.000, 1745.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 20), (c4, 6), &[WireLayoutCommand::MoveTo((-45.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 75.000)), WireLayoutCommand::MoveTo((-200.000, 1775.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 23), (c4, 7), &[WireLayoutCommand::MoveTo((-15.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 105.000)), WireLayoutCommand::MoveTo((-200.000, 1805.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 26), (c4, 8), &[WireLayoutCommand::MoveTo((15.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 105.000)), WireLayoutCommand::MoveTo((200.000, 1805.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 29), (c4, 9), &[WireLayoutCommand::MoveTo((45.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 75.000)), WireLayoutCommand::MoveTo((200.000, 1775.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 32), (c4, 10), &[WireLayoutCommand::MoveTo((75.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 45.000)), WireLayoutCommand::MoveTo((200.000, 1745.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 35), (c4, 11), &[WireLayoutCommand::MoveTo((105.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 15.000)), WireLayoutCommand::MoveTo((200.000, 1715.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 38), (c4, 12), &[WireLayoutCommand::MoveTo((135.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -15.000)), WireLayoutCommand::MoveTo((200.000, 1685.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 41), (c4, 13), &[WireLayoutCommand::MoveTo((165.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -45.000)), WireLayoutCommand::MoveTo((200.000, 1655.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 44), (c4, 14), &[WireLayoutCommand::MoveTo((195.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -75.000)), WireLayoutCommand::MoveTo((200.000, 1625.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c7, 47), (c4, 15), &[WireLayoutCommand::MoveTo((225.000, 1700.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((-0.000, -105.000)), WireLayoutCommand::MoveTo((200.000, 1595.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c2, 0), (c8, 0), &[WireLayoutCommand::MoveTo((-1677.750, 0.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1677.750, 120.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c3, 0), (c8, 1), &[WireLayoutCommand::MoveTo((-1677.750, 250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1677.750, 180.000)), WireLayoutCommand::DontRenderPreviousVertical, WireLayoutCommand::AlignHorizontal]);
	circuit.connect((c8, 2), (c1, 16), &[WireLayoutCommand::MoveTo((-1400.000, 150.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1400.000, 950.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-650.000, 950.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	circuit.connect((c9, 0), (c0, 16), &[WireLayoutCommand::MoveTo((-1400.000, -250.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, 0.000)), WireLayoutCommand::MoveTo((-1400.000, -900.000)), WireLayoutCommand::DontRenderPrevious, WireLayoutCommand::Move((0.000, -0.000)), WireLayoutCommand::MoveTo((650.000, -900.000)), WireLayoutCommand::DontRenderPreviousHorizontal, WireLayoutCommand::AlignVertical]);
	
	circuit.pinify(&mut [c2, c3, c9, c4, c6]);

	circuit
}

pub fn get_register_file_circuit(inner_scale: f64) -> Circuit {
	let mut circuit = Circuit::new();

	let port_pins: Vec<_> = get_pin_coords(0.0, 16, 250.0).iter()
		.map(|x| add!(circuit, Pin, (*x, 250.0 / inner_scale)))
		.collect();

	let address_1_pins: Vec<_> = get_pin_coords(-75.0 / inner_scale, 4, 250.0).iter()
		.map(|y| add!(circuit, Pin, (-500.0 / inner_scale, *y)))
		.collect();

	let address_2_pins: Vec<_> = get_pin_coords(0.0, 4, 250.0).iter()
		.map(|y| add!(circuit, Pin, (-500.0 / inner_scale, *y)))
		.collect();

	let wtb = add!(circuit, Pin, (-500.0 / inner_scale, 75.0 / inner_scale));
	let rfb = add!(circuit, Pin, (-500.0 / inner_scale, 100.0 / inner_scale));
	let clock = add!(circuit, Pin, (-500.0 / inner_scale, 125.0 / inner_scale));

	let mut register_coords = get_pin_coords(0.0, 16, 900.0);

	let registers: Vec<_> = register_coords.iter()
		.map(|x| add!(circuit, Register, (*x, 100.0 / inner_scale)))
		.collect();

	register_coords.pop();

	let junctions: Vec<_> = register_coords.iter()
		.map(|x| add!(circuit, MultiJunction, (*x, 150.0 / inner_scale), 16))
		.collect();

	let clock_junctions: Vec<_> = register_coords.iter()
		.map(|x| add!(circuit, Junction, (*x - 25.0 / inner_scale, 130.0 / inner_scale), 3))
		.collect();

	let decoder1 = add!(circuit, Rom, (-450.0 / inner_scale, -120.0 / inner_scale), 4);
	let decoder2 = add!(circuit, Rom, (-450.0 / inner_scale, -70.0 / inner_scale), 4);

	let decoder_data: Vec<_> = (0..16).map(|i| 1 << (15 - i)).collect();
	circuit.set_memory(decoder1, &decoder_data);
	circuit.set_memory(decoder2, &decoder_data);

	let mult1 = add!(circuit, MultiMultiplexer, (-350.0 / inner_scale, -150.0 / inner_scale), 16);
	let mult2 = add!(circuit, MultiMultiplexer, (-350.0 / inner_scale, -50.0 / inner_scale), 16);

	circuit.connect_groups(
		&(0..4).map(|i| (decoder1, i)).collect::<Vec<_>>(),
		&address_1_pins.iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(0..4).map(|i| (decoder2, i)).collect::<Vec<_>>(),
		&address_2_pins.iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::MoveHorizontal(-100.0),
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit.connect_groups(
		&(4..20).map(|i| (decoder1, i)).collect::<Vec<_>>(),
		&(16..32).map(|i| (mult1, i)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);
	circuit.connect_groups(
		&(4..20).map(|i| (decoder2, i)).collect::<Vec<_>>(),
		&(16..32).map(|i| (mult2, i)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterHorizontal,
			BusLayoutCommand::AlignHorizontal,
		],
	);

	circuit.connect_groups(
		&(33..33+16).rev().map(|i| (mult1, i)).collect::<Vec<_>>(),
		&registers.iter().map(|r| (*r, 34)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::MoveHorizontal(5000.0),
			BusLayoutCommand::MoveVertical(3000.0),
			BusLayoutCommand::AlignVertical,
			BusLayoutCommand::Individual(WireLayoutCommand::DontRenderPrevious),
			BusLayoutCommand::Individual(WireLayoutCommand::MoveHorizontal(-100.0)),
			BusLayoutCommand::Individual(WireLayoutCommand::AlignHorizontal),
		],
	);
	circuit.connect_groups(
		&(33..33+16).rev().map(|i| (mult2, i)).collect::<Vec<_>>(),
		&registers.iter().map(|r| (*r, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::MoveHorizontal(4000.0),
			BusLayoutCommand::MoveVertical(1000.0),
			BusLayoutCommand::AlignVertical,
			BusLayoutCommand::Individual(WireLayoutCommand::DontRenderPrevious),
			BusLayoutCommand::Individual(WireLayoutCommand::MoveHorizontal(-50.0)),
			BusLayoutCommand::Individual(WireLayoutCommand::AlignHorizontal),
		],
	);
	
	circuit.connect_groups(
		&(0..16).map(|i| (junctions[14], i * 3 + 1)).collect::<Vec<_>>(),
		&(0..16).map(|i| (registers[15], i + 17)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::AlignVertical,
		],
	);

	for i in 0..14 {
		circuit.connect_groups(
			&(0..16).map(|idx| (junctions[i], idx * 3 + 1)).collect::<Vec<_>>(),
			&(0..16).map(|idx| (junctions[i + 1], idx * 3 + 2)).collect::<Vec<_>>(),
			&[],
		);
	}

	for i in 0..15 {
		circuit.connect_groups(
			&(0..8).map(|idx| (registers[i], idx + 17)).collect::<Vec<_>>(),
			&(0..8).map(|idx| (junctions[i], idx * 3)).collect::<Vec<_>>(),
			&[
				BusLayoutCommand::MoveVertical(200.0),
				BusLayoutCommand::AlignVertical,
			],
		);
		circuit.connect_groups(
			&(8..16).map(|idx| (registers[i], idx + 17)).collect::<Vec<_>>(),
			&(8..16).map(|idx| (junctions[i], idx * 3)).collect::<Vec<_>>(),
			&[
				BusLayoutCommand::MoveVertical(200.0),
				BusLayoutCommand::AlignVertical,
			],
		);
	}

	circuit.connect_groups(
		&(0..16).map(|i| (junctions[0], i * 3 + 2)).collect::<Vec<_>>(),
		&port_pins.iter().map(|c| (*c, 0)).collect::<Vec<_>>(),
		&[
			BusLayoutCommand::CenterVertical,
			BusLayoutCommand::AlignVertical,
		],
	);

	for i in 0..14 {
		circuit.connect((clock_junctions[i], 1), (clock_junctions[i + 1], 2), &[]);
	}
	for i in 0..15 {
		circuit.connect((clock_junctions[i], 0), (registers[i], 33), &[
			WireLayoutCommand::AlignHorizontal,
		]);
	}

	circuit.connect((clock, 0), (clock_junctions[0], 2), &[
		WireLayoutCommand::CenterHorizontal,
		WireLayoutCommand::AlignHorizontal,
	]);
	circuit.connect((clock_junctions[14], 0), (registers[15], 33), &[
		WireLayoutCommand::MoveHorizontal(900.0),
		WireLayoutCommand::AlignHorizontal,
	]);

	circuit.connect((wtb, 0), (mult1, 32), &[
		WireLayoutCommand::MoveHorizontal(600.0),
		WireLayoutCommand::MoveVertical(-2000.0),
		WireLayoutCommand::MoveHorizontal(800.0),
		WireLayoutCommand::MoveVertical(-1000.0),
		WireLayoutCommand::AlignVertical,
	]);
	circuit.connect((rfb, 0), (mult2, 32), &[
		WireLayoutCommand::MoveHorizontal(800.0),
		WireLayoutCommand::MoveVertical(-1700.0),
		WireLayoutCommand::AlignVertical,
	]);

	circuit
}
