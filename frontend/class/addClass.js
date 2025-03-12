import { addClass, getRooms, getFaculty, getSchedules, addClassScheduleRoom } from '../api/api.js';

let roomsData = [];
let facultyData = [];
let schedulesData = [];

function updateNotificationBanner(message, isSuccess) {
    const notificationBanner = document.getElementById("notificationBanner");
    console.log('notificationBanner:', notificationBanner);

    notificationBanner.textContent = message;

    if (isSuccess) {
        notificationBanner.classList.remove('alert-danger');
        notificationBanner.classList.add('alert-success');
    } else {
        notificationBanner.classList.remove('alert-success');
        notificationBanner.classList.add('alert-danger');
    }

    notificationBanner.classList.remove('d-none');

    setTimeout(() => {
        notificationBanner.classList.add('d-none');
    }, 5000);
}

const getMostRecentClassId = async () => {
    const classes = await getClasses();

    if (classes && classes.length > 0) {
        const mostRecentClass = classes[classes.length - 1];
        return mostRecentClass.id + 1; 
    }

    return 1;
};

document.getElementById('submitClassButton').addEventListener('click', async function(event) {
    event.preventDefault();

    try {
        if (!testTextFields() || !checkDropdownMenus() || !checkCheckboxes() || !checkSelectFields()) {
            throw new Error("Class Form Error");
        } else {
            clearErrors();
        }

        const className = document.getElementById("className").value;
        const classDescription = document.getElementById("classDescription").value;
        const classCode = document.getElementById("classCode").value;
        const classSection = document.getElementById("classSection").value;
        const capacity = document.getElementById("capacity").value;
        const scheduleId = document.getElementById("classScheduleSelect").value;
        const classroomId = document.getElementById("classroomSelect").value;
        const classType = document.querySelector('input[name="classType"]:checked')?.value;
        const classTerm = document.querySelector('input[name="classTerm"]:checked')?.value;

        const classId = await getMostRecentClassId();

        const classData = {
            name: className,
            description: classDescription,
            capacity: parseInt(capacity),
            code: classCode,
            kind: classType,
            section: classSection,
            term: classTerm,
        };

        addClass(classData)
            .then((isSuccess) => {
                const notificationBanner = document.getElementById("notificationBanner");

                if (isSuccess) {
                    updateNotificationBanner("Class has been successfully added!", true);

                    const classScheduleRoomData = {
                        class: classId,
                        room: classroomId,
                        schedule: scheduleId
                    };

                    addClassScheduleRoom(classScheduleRoomData)
                        .then((scheduleRoomSuccess) => {
                            if (scheduleRoomSuccess) {
                                updateNotificationBanner("Class schedule and room mapping successful!", true);
                            } else {
                                updateNotificationBanner("Error: Class schedule-room mapping failed!", false);
                            }
                        })
                        .catch((error) => {
                            console.error("Error mapping class, schedule, and room:", error);
                            updateNotificationBanner("Error: Class schedule-room mapping failed!", false);
                        });

                    document.getElementById("className").value = '';
                    document.getElementById("classDescription").value = '';
                    document.getElementById("classCode").value = '';
                    document.getElementById("classSection").value = '';
                    document.getElementById("capacity").value = '';
                    document.getElementById("facultySelect").value = '';
                    document.getElementById("classScheduleSelect").value = '';
                    document.getElementById("classroomSelect").value = '';
                } else {
                    updateNotificationBanner("Error: Class was not added!", false);
                    throw new Error("Class creation error.");
                }
            })
            .catch((e) => {
                updateNotificationBanner("An error occurred: " + e, false);
                throw new Error("Class creation error.");
            });
    } catch (e) {
        console.log("Caught error: ", e);
    }
});

const populateClassroomDropdown = async () => {
    const classroomDropdown = document.getElementById("classroomSelect");

    try {
        const rooms = await getRooms();
        roomsData = rooms || [];

        if (roomsData.length > 0) {
            roomsData.forEach(room => {
                const roomData = room.data;
                const option = document.createElement("option");
                option.value = room.id; 
                option.textContent = `${roomData.number} - ${roomData.kind} (Capacity: ${roomData.capacity})`;
                classroomDropdown.appendChild(option);
            });
        } else {
            alert("No classrooms available.");
        }
    } catch (error) {
        console.error("Failed to populate classrooms:", error);
    }
};

const populateFacultyDropdown = async () => {
    const facultyDropdown = document.getElementById("facultySelect");

    try {
        const faculty = await getFaculty();
        facultyData = faculty || [];

        if (facultyData.length > 0) {
            facultyData.forEach(facultyMember => {
                const facultyData = facultyMember.data;
                const option = document.createElement("option");
                option.value = facultyMember.id;
                option.textContent = `${facultyData.name} (${facultyData.department})`;
                facultyDropdown.appendChild(option);
            });
        } else {
            alert("No faculty available.");
        }
    } catch (error) {
        console.error("Failed to populate faculty dropdown:", error);
    }
};

const populateScheduleDropdown = async () => {
    const scheduleDropdown = document.getElementById("classScheduleSelect");

    try {
        const schedules = await getSchedules();
        schedulesData = schedules || [];

        if (schedulesData.length > 0) {
            schedulesData.forEach(schedule => {
                const scheduleData = schedule.data;
                const startTime = `${scheduleData.start.hour}:${String(scheduleData.start.minute).padStart(2, '0')}`;
                const endTime = `${scheduleData.end.hour}:${String(scheduleData.end.minute).padStart(2, '0')}`;
                const option = document.createElement("option");
                option.value = schedule.id;
                option.textContent = `${startTime} - ${endTime} (${scheduleData.days})`;
                scheduleDropdown.appendChild(option);
            });
        } else {
            alert("No schedules available.");
        }
    } catch (error) {
        console.error("Failed to populate schedule dropdown:", error);
    }
};

function checkCheckboxes() {
    const classTypeChecked = document.querySelector("input[name='classType']:checked");
    const classTermChecked = document.querySelector("input[name='classTerm']:checked");

    if (!classTypeChecked) {
        setErrorText("classTypeErrorDisplay", "classTypeError", "Please select a class type.");
        return false;
    } else {
        clearError("classTypeError");
    }

    if (!classTermChecked) {
        setErrorText("classTermErrorDisplay", "classTermError", "Please select a class term.");
        return false;
    } else {
        clearError("classTermError");
    }

    return true;
}

function checkSelectFields() {
    const facultySelect = document.getElementById("facultySelect");
    const classSelect = document.getElementById("classSchedule");
    const classroomSelect = document.getElementById("classroomSelect");

    if (facultySelect.value === "") {
        setErrorText("facultySelectErrorDisplay", "facultySelectError", "Please select a faculty member.");
        return false;
    } else {
        clearError("facultySelectError");
    }

    if (classSelect.value === "") {
        setErrorText("classScheduleErrorDisplay", "classScheduleError", "Please select a class schedule.");
        return false;
    } else {
        clearError("classScheduleError");
    }

    if (classroomSelect.value === "") {
        setErrorText("classClassroomErrorDisplay", "classClassroomError", "Please select a classroom.");
        return false;
    } else {
        clearError("classClassroomError");
    }

    return true;
}

function testTextFields() {
    const className = document.getElementById("className");
    const classDescription = document.getElementById("classDescription");
    const capacity = document.getElementById("capacity");
    const classSection = document.getElementById("classSection");

    if (className.value.trim() === "") {
        setErrorText("classNameErrorDisplay", "classNameError", "Class name cannot be empty.");
        return false;
    } else {
        clearError("classNameError");
    }

    if (classDescription.value.trim() === "") {
        setErrorText("classDescriptionErrorDisplay", "classDescriptionError", "Class description cannot be empty.");
        return false;
    } else {
        clearError("classDescriptionError");
    }

    if (classCode.value.trim() === "") {
        setErrorText("classCodeErrorDisplay", "classCodeError", "Class code cannot be empty.");
        return false;
    } else {
        clearError("classCodeError");
    }

    if (classSection.value.trim() === "") {
        setErrorText("classSectionErrorDisplay", "classSectionError", "Class section cannot be empty.");
        return false;
    } else {
        clearError("classNameError");
    }


    if (capacity.value <= 0) {
        setErrorText("classCapacityErrorDisplay", "classCapacityError", "Class capacity must be a number, and cannot be empty or less than 1.");
        return false;
    } else {
        clearError("classCapacityError");
    }

    return true;
}

function checkDropdownMenus() {
    const facultySelect = document.getElementById("facultySelect");
    const classSelect = document.getElementById("classSchedule");
    const classroomSelect = document.getElementById("classroomSelect");

    if (facultySelect.value === "") {
        setErrorText("facultySelectErrorDisplay", "facultySelectError", "Please select a faculty member.");
        return false;
    } else {
        clearError("facultySelectError");
    }

    if (classSelect.value === "") {
        setErrorText("classScheduleErrorDisplay", "classScheduleError", "Please select a class schedule.");
        return false;
    } else {
        clearError("classScheduleError");
    }

    if (classroomSelect.value === "") {
        setErrorText("classClassroomErrorDisplay", "classClassroomError", "Please select a classroom.");
        return false;
    } else {
        clearError("classClassroomError");
    }

    return true;
}

function setErrorText(inputErrorElement, inputErrorText, text) {
    const errorElement = document.getElementById(inputErrorElement);
    const errorText = document.getElementById(inputErrorText);

    errorElement.innerHTML = text;
    errorText.style.display = "inline-block";
}

function clearError(inputErrorText) {
    const errorText = document.getElementById(inputErrorText);
    if (errorText) {
        errorText.style.display = "none";
    }
}

function clearErrors() {
    const errorElements = ['classNameError', 'classDescriptionError', 'classCodeError', 'classCapacityError', 'facultySelectError', 'classScheduleError', 'classClassroomError', 'classTypeError', 'classTermError', 'classFeaturesError'];

    errorElements.forEach(id => {
        clearError(id);
    });
}

document.addEventListener('DOMContentLoaded', function() {
    populateClassroomDropdown();
    populateFacultyDropdown();
    populateScheduleDropdown();
}); 

$(document).ready(function() {
    $('#facultySelect').select2({
        placeholder: "Select an option",
        allowClear: true
    });

    $('#classroomSelect').select2({
        placeholder: "Select an option",
        allowClear: true
    });

    $('#classScheduleSelect').select2({
        placeholder: "Select an option",
        allowClear: true
    });
});