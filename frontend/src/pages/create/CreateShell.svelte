<script>
  import { createEventDispatcher } from 'svelte';
  import { ArrowLeft } from 'lucide-svelte';
  import Stepper from '../../components/ui/Stepper.svelte';
  import Button from '../../components/ui/Button.svelte';

  const dispatch = createEventDispatcher();

  // formData mirrors the shape CreateWentu.svelte submits today.
  // Tasks 6-8 will populate individual fields as each step is built.
  let formData = {
    title: '',
    description: '',
    creatorName: '',
    dateRangeStart: null,
    dateRangeEnd: null,
    prefDeadline: '',
    prefDeadlineTime: '23:59',
    enableTimeSlots: false,
    timezone: 'Europe/London',
    dayTimeSlots: {},
    excludedDays: [],
  };

  const ALL_STEPS = [
    { id: 'basics', label: 'Basics' },
    { id: 'dates', label: 'Dates' },
    { id: 'timeslots', label: 'Time slots' },
    { id: 'review', label: 'Review' },
  ];

  let currentStep = 'basics';
  let completed = new Set();

  $: steps = formData.enableTimeSlots
    ? ALL_STEPS
    : ALL_STEPS.filter((s) => s.id !== 'timeslots');

  $: currentIndex = steps.findIndex((s) => s.id === currentStep);
  $: isFirstStep = currentIndex === 0;
  $: isLastStep = currentIndex === steps.length - 1;

  function goNext() {
    const next = steps[currentIndex + 1];
    if (!next) return;
    completed = new Set([...completed, currentStep]);
    currentStep = next.id;
  }

  function goBack() {
    const prev = steps[currentIndex - 1];
    if (prev) currentStep = prev.id;
  }

  function handleStepperNavigate(event) {
    const targetId = event.detail.id;
    if (completed.has(targetId)) currentStep = targetId;
  }

  function goHome() {
    dispatch('navigate', { page: 'home' });
  }
</script>

<div class="max-w-2xl mx-auto">
  <button
    type="button"
    class="inline-flex items-center gap-1 text-text-secondary hover:text-text-primary text-sm mb-4 cursor-pointer"
    on:click={goHome}
  >
    <ArrowLeft size={16} aria-hidden="true" />
    Back to home
  </button>

  <h1 class="text-2xl sm:text-3xl font-bold text-text-primary mb-4">Create a Wentu</h1>

  <div class="mb-6">
    <Stepper {steps} current={currentStep} {completed} on:navigate={handleStepperNavigate} />
  </div>

  <!--
    Step content slots. Tasks 6-8 will replace these placeholders with the
    real step components (StepBasics, StepDates, StepTimeSlots, StepReview).
  -->
  <section aria-labelledby="step-heading" class="bg-surface-card border border-border-subtle rounded-lg p-4 sm:p-6">
    <h2 id="step-heading" class="text-lg font-semibold text-text-primary mb-4">
      {steps[currentIndex]?.label ?? ''}
    </h2>

    {#if currentStep === 'basics'}
      <p class="text-text-muted text-sm">Placeholder — StepBasics lands in Task 6.</p>
    {:else if currentStep === 'dates'}
      <p class="text-text-muted text-sm">Placeholder — StepDates lands in Task 7.</p>
      <label class="mt-4 flex items-center gap-2 text-sm text-text-secondary">
        <input type="checkbox" bind:checked={formData.enableTimeSlots} />
        Add time slots (enables the Time slots step)
      </label>
    {:else if currentStep === 'timeslots'}
      <p class="text-text-muted text-sm">Placeholder — StepTimeSlots lands in Task 8.</p>
    {:else if currentStep === 'review'}
      <p class="text-text-muted text-sm">Placeholder — StepReview lands in Task 8.</p>
    {/if}
  </section>

  <div class="flex items-center justify-between mt-6">
    <Button variant="secondary" disabled={isFirstStep} on:click={goBack}>Back</Button>
    <Button variant="primary" disabled={isLastStep} on:click={goNext}>Next</Button>
  </div>
</div>
