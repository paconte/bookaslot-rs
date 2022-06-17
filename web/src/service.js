import dateFormat from "dateformat";

const hourlyReservationsUrl = 'https://195.201.148.68/reservations/getBookings4';
const dailyReservationsUrl = 'https://195.201.148.68/reservations/getBookings6';
//const dailyReservationsUrl = 'http://0.0.0.0:8000/getBookings6';
const addReservationUrl = 'https://195.201.148.68/reservations/addReservations';
//const addReservationUrl = 'http://0.0.0.0:8000/addReservations';

const data = [
	//TODO: move to testing
	[{id:1, time:"09:00 - 10:00"}, {id:2, state:"FREE"}, {id:3, state:"FREE"}],
	[{id:4, time:"10:00 - 11:00"}, {id:5, state:"FREE"}, {id:6, state:"FREE"}],
	[{id:7, time:"11:00 - 13:00"}, {id:8, state:"FREE"}, {id:9, state:"FREE"}],
	[{id:10, time:"12:00 - 13:00"}, {id:11, state:"FREE"}, {id:12, state:"FREE"}],
	[{id:13, time:"13:00 - 14:00"}, {id:14, state:"FREE"}, {id:15, state:"FREE"}],
];


const data2 = [
	{
		date: "TODAY",
		data: [
			//TODO: move to testing
			{time:{init:1641027600, end:1641031200}, items: [{id:2, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:3, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:5, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:6, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:8, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:9, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:11, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:12, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:14, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:15, state:"FREE", time:{init:1641027600, end:1641031200}}]},
		]
	},
	{
		date: "TOMORROW",
		data: [
			//TODO: move to testing
			{time:{init:1641027600, end:1641031200}, items: [{id:2, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:3, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:5, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:6, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:8, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:9, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:11, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:12, state:"FREE", time:{init:1641027600, end:1641031200}}]},
			{time:{init:1641027600, end:1641031200}, items: [{id:14, state:"FREE", time:{init:1641027600, end:1641031200}}, {id:15, state:"FREE", time:{init:1641027600, end:1641031200}}]},
		]
	},
]


export const state = {
	FREE: "FREE",
	BOOKED: "BOOKED",
	TO_BE_BOOKED: "TOBEBOOKED",
}


async function fetchReservations(url) {
	//await debugReservations(url);
	const response = await fetch(url);
	const reservations = await response.json();
	return reservations;

}


async function debugReservations(url) {
	fetch(url)
	.then(res => {
		console.log(res);
		res.json();
	}).catch(error => console.error('Error:', error))
	.then(response => console.log('Success:', response));
}


function formatMinutes(minutes) {
	let result = minutes;
	if (minutes < 10) {
		result = "0" + minutes;
	}
	return result;
}


function formatTime(timestamp) {
	let date = new Date(timestamp);
	let minutes = formatMinutes(date.getMinutes());
	return date.getHours() + ":" + minutes;
}


function formatDayReservations(reservations) {
	let result = Array();

	reservations.forEach(element => {
		let time =
			formatTime(element.time.init * 1000)
			+ " - "
			+ formatTime(element.time.end * 1000);
		let item = {id: -1, timeString: time};
		let row = [item];
		row = row.concat(element.items);
		result.push(row);
	});

	return result;
}


export async function getReservationsTable() {
	const reservations = await fetchReservations(hourlyReservationsUrl);
	return formatDayReservations(reservations);
}


export function getReservationsTable2() {
	let result = Array();
	const data = data2;

	data.forEach(item => {
		let newItem = {
			date: item.date,
			data: formatDayReservations(item.data)
		};
		result.push(newItem);
	});

	return result;
}


export async function getReservationsTable3() {
	let result = Array();
	let data = await fetchReservations(dailyReservationsUrl);

	data.forEach(item => {
		let newItem = {
			date: item.date,
			data: formatDayReservations(item.data)
		};
		result.push(newItem);
	});

	return result;
}


export async function addReservations(slots) {
	const response = await fetch(addReservationUrl, {
		method: 'POST', // or 'PUT'
		mode: 'cors', // no-cors, cors, same-origin
		body: JSON.stringify(slots), // data can be `string` or {object}!
		headers: {'Content-Type': 'application/json'}
	});

	if (response.status === 201) {
		return await response.json();
	} else if (response.status === 200) {
		const data = await response.json();
		throw new Error(data.details);
	} else {
		throw new Error(response.statusText);
	}
}


export function findIndex(data, date) {
	return data.findIndex(element => element.date == date);
}


export function formatDate(thedate) {
	return dateFormat(thedate, "yyyy-mm-dd");
}

export function formatDate2(thedate) {
	return dateFormat(thedate, "dddd, d.m.yy");
}