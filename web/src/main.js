import App from './BookingApp.svelte';

const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;