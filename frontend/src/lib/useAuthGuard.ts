import { useEffect, useState } from 'react';
import { api } from './api';

export function useAuthGuard() {
	const [ready, setReady] = useState(false);

	useEffect(() => {
		api.auth
			.me()
			.then(() => setReady(true))
			.catch(() => {
				window.location.href = '/login';
			});
	}, []);

	return ready;
}
