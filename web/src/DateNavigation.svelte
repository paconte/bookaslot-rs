<script>
	import { selectedDate } from './stores.js';
	import {findIndex, formatDate, formatDate2} from './service.js';

	export let data;

  let current = formatDate(new Date());
  selectedDate.set(current);


  let date;

  selectedDate.subscribe(value => {
		date = value;
	});

  function handleNext(days) {
    let index = findIndex(data, date);
    if (index < data.length-days) {
      selectedDate.set(data[index+days].date);
    }
	}

  function handlePrev(days) {
    let index = findIndex(data, date);
    if (index > days-1) {
      selectedDate.set(data[index-days].date);
    }
	}
</script>

<div class="flex flex-row mx-auto">

  <button type="button" on:click={() => handlePrev(7)} class="bg-gray-800 text-white rounded-l-md border-r border-gray-100 py-2 hover:bg-red-700 hover:text-white px-3">
        <div class="flex flex-row align-middle">
          <svg class="w-5 mr-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M7.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l2.293 2.293a1 1 0 010 1.414z" clip-rule="evenodd"></path>
          </svg>
          <svg class="w-5 mr-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M7.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l2.293 2.293a1 1 0 010 1.414z" clip-rule="evenodd"></path>
          </svg>
        </div>
  </button>

  <button type="button" on:click={() => handlePrev(1)} class="bg-gray-800 text-white rounded-l-md border-r border-gray-100 py-2 hover:bg-red-700 hover:text-white px-3">
        <div class="flex flex-row align-middle">
          <svg class="w-5 mr-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M7.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l2.293 2.293a1 1 0 010 1.414z" clip-rule="evenodd"></path>
          </svg>
          <!-- <p class="ml-2">Prev</p> -->
        </div>
  </button>

  <button type="button" class="bg-red-700 bg-gray-800 text-white rounded-l-md rounded-r-md border-r border-gray-100 py-2 px-3 w-60">
        {formatDate2(date)}
  </button>

  <button type="button" on:click={() => handleNext(1)} class="flex-end bg-gray-800 text-white rounded-r-md py-2 border-l border-gray-200 hover:bg-red-700 hover:text-white px-3">
        <div class="flex flex-row align-middle">
          <!-- <span class="mr-2">Next</span> -->
          <svg class="w-5 ml-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
          </svg>
        </div>
  </button>

  <button type="button" on:click={() => handleNext(7)} class="flex-end bg-gray-800 text-white rounded-r-md py-2 border-l border-gray-200 hover:bg-red-700 hover:text-white px-3">
        <div class="flex flex-row align-middle">
          <svg class="w-5 ml-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
          </svg>
          <svg class="w-5 ml-2" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
          </svg>
        </div>
  </button>

</div>

