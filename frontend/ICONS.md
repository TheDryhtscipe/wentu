# Icon Library - Lucide Svelte

This project uses [Lucide](https://lucide.dev/) for icons, replacing emoji characters for a more consistent, professional appearance.

## Installation

Already installed:
```bash
npm install lucide-svelte
```

## Usage

### Basic Example

```svelte
<script>
  import { ArrowLeft, Check, X } from 'lucide-svelte';
</script>

<button>
  <ArrowLeft size={16} />
  Back
</button>
```

### Common Props

- `size` - Icon size in pixels (default: 24)
- `color` - Icon color (default: currentColor)
- `strokeWidth` - Line thickness (default: 2)
- `class` - CSS classes for styling

### Examples in This Project

**Navigation arrows:**
```svelte
import { ChevronLeft, ChevronRight } from 'lucide-svelte';

<ChevronLeft size={18} />
<ChevronRight size={18} />
```

**Back buttons:**
```svelte
import { ArrowLeft } from 'lucide-svelte';

<button class="flex items-center gap-1">
  <ArrowLeft size={16} />
  Back to home
</button>
```

**Status indicators:**
```svelte
import { XCircle, CheckCircle } from 'lucide-svelte';

<XCircle size={14} class="text-error" />
<CheckCircle size={14} class="text-success" />
```

**Info/tips:**
```svelte
import { Lightbulb } from 'lucide-svelte';

<Lightbulb size={16} class="text-accent" />
```

## Commonly Used Icons

- **Navigation**: `ArrowLeft`, `ArrowRight`, `ChevronLeft`, `ChevronRight`, `ChevronDown`, `ChevronUp`
- **Actions**: `Plus`, `Minus`, `X`, `Check`, `Edit`, `Trash2`, `Save`, `Copy`
- **Status**: `CheckCircle`, `XCircle`, `AlertCircle`, `Info`, `AlertTriangle`
- **UI**: `Menu`, `Search`, `Settings`, `User`, `Calendar`, `Clock`
- **Misc**: `Lightbulb`, `Star`, `Heart`, `Share2`, `Download`, `Upload`

## Browse All Icons

Visit [lucide.dev/icons](https://lucide.dev/icons) to browse all available icons.

## Styling with Tailwind

Since icons use `currentColor` by default, you can style them with Tailwind text color utilities:

```svelte
<Check class="text-success" />
<X class="text-error" />
<Info class="text-accent" />
```
