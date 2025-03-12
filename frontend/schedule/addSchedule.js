import { addSchedule } from '../api/api.js';

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

document.getElementById('submitScheduleButton').addEventListener('click', function(event) {
    event.preventDefault();

    try {
        if (!testScheduleFields()) {
            throw new Error("Schedule Form Error");
        } else {
            clearErrors();
        }

        const startHour = parseInt(document.getElementById("startHour").value);
        const startMinute = parseInt(document.getElementById("startMinute").value);
        const endHour = parseInt(document.getElementById("endHour").value);
        const endMinute = parseInt(document.getElementById("endMinute").value);
        
        const selectedDays = [];
        if (document.getElementById("mon").checked) selectedDays.push("Monday");
        if (document.getElementById("tue").checked) selectedDays.push("Tuesday");
        if (document.getElementById("wed").checked) selectedDays.push("Wednesday");
        if (document.getElementById("thu").checked) selectedDays.push("Thursday");
        if (document.getElementById("fri").checked) selectedDays.push("Friday");

        if (selectedDays.length === 0) {
            throw new Error("Please select at least one day.");
        }

        const scheduleData = {
            start: {
                hour: startHour,
                minute: startMinute
            },
            end: {
                hour: endHour,
                minute: endMinute
            },
            days: selectedDays.join(", ") 
        };

        addSchedule(scheduleData)
            .then((isSuccess) => {
                const notificationBanner = document.getElementById("notificationBanner");

                if (isSuccess) {
                    updateNotificationBanner("Schedule has been successfully added!", true);

                    // Reset form fields after successful submission
                    document.getElementById("startHour").value = '';
                    document.getElementById("startMinute").value = '';
                    document.getElementById("endHour").value = '';
                    document.getElementById("endMinute").value = '';

                    // Uncheck all days
                    document.getElementById("mon").checked = false;
                    document.getElementById("tue").checked = false;
                    document.getElementById("wed").checked = false;
                    document.getElementById("thu").checked = false;
                    document.getElementById("fri").checked = false;
                } else {
                    updateNotificationBanner("Error: Schedule was not added!", false);
                    throw new Error("Schedule creation error.");
                }
            })
            .catch((e) => {
                updateNotificationBanner("An error occurred: " + e, false);
                throw new Error("Schedule creation error.");
            });

    } catch (e) {
        console.log("Caught error: ", e);
    }
});

function testScheduleFields() {
    const startHour = document.getElementById("startHour");
    const startMinute = document.getElementById("startMinute");
    const endHour = document.getElementById("endHour");
    const endMinute = document.getElementById("endMinute");
    const days = document.querySelectorAll("input[name='days']:checked");

    const startTotalMinutes = (parseInt(startHour.value) * 60) + parseInt(startMinute.value);
    const endTotalMinutes = (parseInt(endHour.value) * 60) + parseInt(endMinute.value);

    if (startTotalMinutes >= endTotalMinutes) {
        setErrorText("endHourErrorDisplay", "endHourError", "End time cannot be before or equal to start time.");
        return false;
    } else {
        clearError("endHourError");
    }

    if (startHour.value.trim() === "" || startHour.value < 0 || startHour.value > 23) {
        setErrorText("startHourErrorDisplay", "startHourError", "Start hour must be between 0 and 23.");
        return false;
    } else {
        clearError("startHourError");
    }

    if (startMinute.value.trim() === "" || startMinute.value < 0 || startMinute.value > 59) {
        setErrorText("startMinuteErrorDisplay", "startMinuteError", "Start minute must be between 0 and 59.");
        return false;
    } else {
        clearError("startMinuteError");
    }

    if (endHour.value.trim() === "" || endHour.value < 0 || endHour.value > 23) {
        setErrorText("endHourErrorDisplay", "endHourError", "End hour must be between 0 and 23.");
        return false;
    } else {
        clearError("endHourError");
    }

    if (endMinute.value.trim() === "" || endMinute.value < 0 || endMinute.value > 59) {
        setErrorText("endMinuteErrorDisplay", "endMinuteError", "End minute must be between 0 and 59.");
        return false;
    } else {
        clearError("endMinuteError");
    }

    if (days.length === 0) {
        setErrorText("daysErrorDisplay", "daysError", "Please select at least one day.");
        return false;
    } else {
        clearError("daysError");
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
    errorText.style.display = "none";
}

function clearErrors() {
    const errorElements = ['startHourError', 'startMinuteError', 'endHourError', 'endMinuteError', 'daysError'];
    errorElements.forEach(id => {
        clearError(id);
    });
}