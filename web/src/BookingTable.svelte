<script>
	import { bookings, selectedDate } from './stores.js';
	import { addReservations, findIndex, state } from './service.js';
	import { onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';


	export let data;

	//let columns = ["Time", "Pista 1", "Pista 2", "Pista 3", "Pista 4", "Pista 5", "Pista 6"]
	let columns = ["Time", "Pista 1", "Pista 2"]
	let currentData;
	let index;
	let bookingsLength = 0; // size of the bookings set in the store
	let bookSlotsPromise;


	/* STORE subscriptions */
	const unsubscribe1 = bookings.subscribe(collection => {
		bookingsLength = collection.length;
	});

	const unsubscribe2 = selectedDate.subscribe(value => {
		index = findIndex(data, value);
		currentData = data[index].data;
	});

	onDestroy(unsubscribe1);
	onDestroy(unsubscribe2);

	/* STORE methods */
	const addBooking = (item) => {
		// add booking to the store
		$bookings = [...$bookings, item];
	};

	const deleteBooking = (item) => {
		// delete booking from the store
		$bookings = $bookings.filter(element => element != item);
	}


	/* COMPONENT methods */
	function getCellBackgroundColor(cell) {
		if (!cell.hasOwnProperty('state')) {
			return "";
		}

		let result;

		switch (cell.state) {
			case state.FREE:
				result = "bg-green-100";
				break;
			case state.BOOKED:
				result = "bg-red-100";
				break;
			case state.TO_BE_BOOKED:
				result = "bg-yellow-100";
				break;
			case undefined:
				result = "";
				break;
			default:
				const text = 'Invalid argument state: ' + state;
				console.error(text)
				throw new TypeError(text);
		}

		return result;
	}

	function updateItemState(item, state) {
		currentData.forEach(row => {
		  // find item
			var index = row.map((cell) => cell.id).indexOf(item.id);
			// update data
			if (index !== -1) {
				row[index].state = state;
				row[index].bg = getCellBackgroundColor(state);
				return;
			};
		});
		currentData = currentData;
	}

	function bookItem(item) {
		switch (item.state) {
			case state.TO_BE_BOOKED:
				updateItemState(item, state.FREE);
				deleteBooking(item);
				break;
			case state.FREE:
				updateItemState(item, state.TO_BE_BOOKED);
				addBooking(item);
				break;
		}
	}

	function bookSlots() {
		bookSlotsPromise = addReservations($bookings);
		$bookings.forEach(item => bookItem(item));
	}
</script>

<div class="w-full mb-8 overflow-hidden rounded-lg shadow-lg">
	<div class="w-full overflow-x-auto">
		<table class="w-full">
			<thead>
				<tr class="text-md font-semibold tracking-wide text-left text-gray-900 bg-gray-100 uppercase border-b border-gray-600">
					{#each columns as column, i}
						<th class="px-4 py-3 {i===0 ? "w-40" : ""}">{column}</th>
						<!-- <th class="px-4 py-3">{column}</th> -->
					{/each}
				</tr>
			</thead>

			<tbody class="bg-white">
				{#each currentData as row}
					<tr class="text-gray-700">
						{#each row as cell}
							<td on:click={() => bookItem(cell)} class="px-4 py-3 border {getCellBackgroundColor(cell)}">
								{(cell.timeString) ? cell.timeString : ""}
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>


<p>
[
{#each Array.from($bookings) as item}
	({item.id}, {item.state}),
{/each}
]
</p>

<br/>

{#if bookingsLength > 0}
	<button on:click={bookSlots} type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center mr-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
		<svg class="mr-2 -ml-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
			<path d="M3 1a1 1 0 000 2h1.22l.305 1.222a.997.997 0 00.01.042l1.358 5.43-.893.892C3.74 11.846 4.632 14 6.414 14H15a1 1 0 000-2H6.414l1-1H14a1 1 0 00.894-.553l3-6A1 1 0 0017 3H6.28l-.31-1.243A1 1 0 005 1H3zM16 16.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM6.5 18a1.5 1.5 0 100-3 1.5 1.5 0 000 3z">
			</path>
		</svg>
		Book now
	</button>
{:else}
	<button disabled type="button" class="text-gray-900 bg-white border border-gray-300 hover:bg-gray-100 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center mr-2 dark:bg-gray-600 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-700 dark:focus:ring-gray-800">
		<svg class="mr-2 -ml-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
			<path d="M3 1a1 1 0 000 2h1.22l.305 1.222a.997.997 0 00.01.042l1.358 5.43-.893.892C3.74 11.846 4.632 14 6.414 14H15a1 1 0 000-2H6.414l1-1H14a1 1 0 00.894-.553l3-6A1 1 0 0017 3H6.28l-.31-1.243A1 1 0 005 1H3zM16 16.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM6.5 18a1.5 1.5 0 100-3 1.5 1.5 0 000 3z">
			</path>
		</svg>
		Book now
	</button>
{/if}


<br/><br/><br/>


{#await bookSlotsPromise}
  <p>...loading</p>
{:then apiResponse}
  <p>{apiResponse ? `sucess` : ''}</p>
{:catch error}
  <div transition:fade class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
	<strong class="font-bold">Error!</strong>
	<span class="block sm:inline">{error.message}</span>
	<span class="absolute top-0 bottom-0 right-0 px-4 py-3">
	  <svg class="fill-current h-6 w-6 text-red-500" role="button" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><title>Close</title><path d="M14.348 14.849a1.2 1.2 0 0 1-1.697 0L10 11.819l-2.651 3.029a1.2 1.2 0 1 1-1.697-1.697l2.758-3.15-2.759-3.152a1.2 1.2 0 1 1 1.697-1.697L10 8.183l2.651-3.031a1.2 1.2 0 1 1 1.697 1.697l-2.758 3.152 2.758 3.15a1.2 1.2 0 0 1 0 1.698z"/></svg>
	</span>
  </div>
{/await}


<!--
class="py-2.5 px-5 mr-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700 inline-flex items-center">

<style>
	@import url("https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css");
</style>
-->