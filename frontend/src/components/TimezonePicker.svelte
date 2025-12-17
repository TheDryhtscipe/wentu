<script>
  import { createEventDispatcher } from 'svelte';
  import { Globe } from 'lucide-svelte';

  const dispatch = createEventDispatcher();
  export let selectedTimezone = 'Europe/London';

  const timezones = [
    { value: 'Europe/London', label: 'UK (London)' },
    { value: 'Europe/Paris', label: 'Central Europe (Paris)' },
    { value: 'America/New_York', label: 'US East (New York)' },
    { value: 'America/Chicago', label: 'US Central (Chicago)' },
    { value: 'America/Los_Angeles', label: 'US West (Los Angeles)' },
    { value: 'Asia/Tokyo', label: 'Japan (Tokyo)' },
    { value: 'Australia/Sydney', label: 'Australia (Sydney)' },
  ];

  function handleChange(e) {
    dispatch('change', e.target.value);
  }
</script>

<div>
  <label class="flex items-center gap-2 text-text-primary font-medium mb-2">
    <Globe size={18} />
    Timezone
  </label>
  <select class="input w-full" bind:value={selectedTimezone} on:change={handleChange}>
    {#each timezones as tz}
      <option value={tz.value}>{tz.label}</option>
    {/each}
  </select>
  <p class="text-text-secondary text-sm mt-1">Time slots will be shown in this timezone</p>
</div>
