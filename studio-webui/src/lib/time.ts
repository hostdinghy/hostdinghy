function pad(n: number): string {
	return String(n).padStart(2, '0');
}

/** returns the date in the format YYYY-MM-DD */
export function browserDate(date: Date) {
	return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

/** returns the time in the format HH:MM */
export function browserTime(date: Date) {
	return `${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

/** returns the date and time in the format YYYY-MM-DD HH:MM */
export function browserDateTime(date: Date) {
	return `${browserDate(date)} ${browserTime(date)}`;
}

/** returns the date and time in a path safe format YYYY-MM-DD_HH-MM */
export function browserDateTimePathSafe(date: Date): string {
	return `${browserDate(date)}_${pad(date.getHours())}-${pad(date.getMinutes())}`;
}
