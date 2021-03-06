import { LeapDashboard } from './components/leap-dashboard';
import { LeapApp } from './components/leap-app';
import { LeapCoursesList } from './components/leap-courses-list';
import { LeapCourseDetail } from './components/leap-course-detail';
import { LeapModule } from './components/leap-module';
import { LeapEmtpyPlacholder } from './components/leap-empty-placeholder';

customElements.define('leap-course-detail', LeapCourseDetail);
customElements.define('leap-courses-list', LeapCoursesList);
customElements.define('leap-app', LeapApp);
customElements.define('leap-dashboard', LeapDashboard);
customElements.define('leap-module', LeapModule);
customElements.define('leap-empty-placeholder', LeapEmtpyPlacholder);