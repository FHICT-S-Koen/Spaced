export function Background() {
  // The following is required to determine how big the pattern should be so how many dots you will see
  // The size of these dot should be a repeating pattern > 10 > 11 > 15 > 10 > 13 > 17 > 11 > 15 > 20 > 13 > 17 > 22 > 30 > 40 (full zoom)
  return (
    <svg class="absolute z-0 h-full w-full">
      <pattern
        id="background"
        width="13"
        height="13"
        patternUnits="userSpaceOnUse"
      >
        <circle cx="0.7" cy="0.7" r="0.7" fill="#363636" />
      </pattern>
      <rect x="0" y="0" fill="url(#background)" class="h-full w-full" />
    </svg>
  );
}
