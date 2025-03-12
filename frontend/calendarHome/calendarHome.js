const sampleEvents = {
    "Monday": {
        "8:00 AM - 9:15 AM": [
            { course: "CSCI1010", status: "normal", details: "Introduction to Programming." },
            { course: "MATH1010", status: "warning", details: "Basic Calculus." }
        ],
        "9:15 AM - 10:30 AM": [
            { course: "ENGR1001", status: "danger", details: "Fundamentals of Engineering." }
        ],
        "10:30 AM - 11:45 AM": [
            { course: "PHYS1010", status: "normal", details: "Physics I." }
        ],
    },
    "Tuesday": {
        "9:15 AM - 10:30 AM": [
            { course: "BIO1001", status: "normal", details: "General Biology." }
        ],
        "10:30 AM - 11:45 AM": [
            { course: "CSCI1020", status: "warning", details: "Data Structures." }
        ],
    },
    "Wednesday": {
        "8:00 AM - 9:15 AM": [
            { course: "CSCI2030", status: "danger", details: "Discrete Structures." }
        ],
        "9:15 AM - 10:30 AM": [
            { course: "ENGR1020", status: "normal", details: "Engineering Design." }
        ],
    },
    "Thursday": {
        "10:30 AM - 11:45 AM": [
            { course: "CSCI3000", status: "normal", details: "Algorithms." }
        ],
        "11:45 AM - 1:00 PM": [
            { course: "BIO2020", status: "danger", details: "Advanced Biology." }
        ],
    },
    "Friday": {
        "8:00 AM - 9:15 AM": [
            { course: "MATH2020", status: "warning", details: "Advanced Calculus." }
        ],
        "9:15 AM - 10:30 AM": [
            { course: "PHYS2020", status: "normal", details: "Advanced Physics." }
        ],
        "10:30 AM - 11:45 AM": [
            { course: "ENGR3000", status: "normal", details: "Electromagnetism." }
        ],
    },
};

const calendarView = document.getElementById("calendarView");
const calendarTitle = document.getElementById("calendarTitle");
const backToWeeklyBtn = document.getElementById("backToWeeklyBtn");

const switchToDailyView = (day) => {
    const timeSlots = Object.keys(sampleEvents[day]);
    calendarView.innerHTML = "";
    timeSlots.forEach(time => {
        const timeDiv = document.createElement("div");
        timeDiv.classList.add("calendar-day");
        timeDiv.classList.add("calendar-day-time");
        timeDiv.innerHTML = `<strong>${time}</strong><br>`;
        timeDiv.setAttribute('data-time', time);
        timeDiv.setAttribute('data-day', day);

        const eventsContainer = document.createElement("div");
        eventsContainer.classList.add("events-container");

        sampleEvents[day][time].forEach((event, index) => {
            const eventDiv = document.createElement("div");
            eventDiv.classList.add("event");
            eventDiv.textContent = event.course;
            eventDiv.draggable = true;
            eventDiv.dataset.day = day;
            eventDiv.dataset.time = time;

            if (event.status === "warning") {
                eventDiv.classList.add("event-warning");
            } else if (event.status === "danger") {
                eventDiv.classList.add("event-danger");
            }

            eventDiv.addEventListener("click", handleClickEvent);
            eventDiv.addEventListener("dragstart", handleDragStart);

            eventsContainer.appendChild(eventDiv);
        });

        timeDiv.appendChild(eventsContainer);
        calendarView.appendChild(timeDiv);

        timeDiv.addEventListener("dragover", handleDragOver);
        timeDiv.addEventListener("drop", handleDrop);
    });

    calendarTitle.innerText = `${day} View`;
    backToWeeklyBtn.style.display = "inline-block"; 
};

const handleDragStart = (e) => {
    e.dataTransfer.setData("text", JSON.stringify({
        course: e.target.textContent,
        day: e.target.dataset.day,
        time: e.target.dataset.time,
        status: e.target.classList.contains("event-warning") ? "warning" : e.target.classList.contains("event-danger") ? "danger" : "normal"
    }));
};

const handleDragOver = (e) => {
    e.preventDefault();
    e.target.classList.add("drag-over");
};

const handleDrop = (e) => {
    e.preventDefault();
    e.target.classList.remove("drag-over");

    if (!e.target.classList.contains("calendar-day-time")) return;

    const dropTime = e.target.dataset.time;
    const dropDay = e.target.dataset.day;

    const data = e.dataTransfer.getData("text");
    const { course, day, time, status } = JSON.parse(data);

    const oldTimeSlot = sampleEvents[day][time];
    sampleEvents[day][time] = oldTimeSlot.filter(event => event.course !== course);

    if (!sampleEvents[dropDay]) {
        sampleEvents[dropDay] = {};
    }

    if (!sampleEvents[dropDay][dropTime]) {
        sampleEvents[dropDay][dropTime] = [];
    }

    sampleEvents[dropDay][dropTime].push({
        course: course,
        status: status,
        details: "Details of the class"
    });

    switchToDailyView(dropDay);
};

const handleClickEvent = (e) => {
    const course = e.target.textContent;
    const day = e.target.dataset.day;
    const time = e.target.dataset.time;

    const event = sampleEvents[day][time].find(e => e.course === course);

    document.getElementById("className").textContent = event.course;
    document.getElementById("classStatus").textContent = event.status;
    document.getElementById("classDetails").textContent = event.details;

    const myModal = new bootstrap.Modal(document.getElementById("classModal"));
    myModal.show();
};

const generateWeeklyView = () => {
    const daysOfWeek = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    const hours = ["8:00 AM - 9:15 AM", "9:15 AM - 10:30 AM", "10:30 AM - 11:45 AM", "11:45 AM - 1:00 PM"];

    calendarView.innerHTML = '';
    daysOfWeek.forEach((day) => {
        const dayDiv = document.createElement("div");
        dayDiv.classList.add("calendar-day");
        dayDiv.innerHTML = `<strong>${day}</strong>`;
        
        let hasEventsForDay = false;

        hours.forEach((hour) => {
            if (sampleEvents[day] && sampleEvents[day][hour]) {
                const hourDiv = document.createElement("div");
                hourDiv.classList.add("calendar-day-time");
                
                const eventsForHour = sampleEvents[day][hour];

                if (eventsForHour.length > 0) {
                    hourDiv.innerHTML = `${hour}: ${eventsForHour.length} class(es)`;
                    dayDiv.appendChild(hourDiv);
                    hasEventsForDay = true;
                }
            }
        });

        if (hasEventsForDay) {
            dayDiv.addEventListener("click", () => switchToDailyView(day));
            calendarView.appendChild(dayDiv);
        }
    });

    backToWeeklyBtn.style.display = "none";
    calendarTitle.innerText = "Weekly View";
};

const goBackToWeeklyView = () => {
    generateWeeklyView();
};

generateWeeklyView();

backToWeeklyBtn.addEventListener("click", goBackToWeeklyView);