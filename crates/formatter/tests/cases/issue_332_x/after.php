<?php

use Illuminate\Support\Facades\Config;

uses()
    ->beforeEach(function () {
        Config::set('auto-router.token', '1234');
    })
    ->in(__DIR__);

it('always loads relationships', function () {
    $report = ReportFactory::new()->create();

    expect($report->getEagerLoads())
        ->toHaveKeys(['user', 'files', 'files.employee']);
});

/** @var Collection<CategoryData> */
$categories = $this->connector->send(new ListEmployeeFilesRequest(
    employeeId: $file->employee->bamboo_id,
))->dtoOrFail();

Mail::send(new BookingUpdatesReportMailable(
    dispatch: $dispatch,
));

return new JobOffers(
    spontaneous_application_url: collect($offers)->where('id', $id)->first()?->url,
    job_offers: collect($offers)
        ->filter(fn (JobData $data) => $data->id !== $id)
        ->map(fn (JobData $data) => new JobData(
            $data->id,
            $data->title,
            $data->city,
            $data->country,
            $data->url,
            $employmentTypes[$data->id] ?? null,
            $data->created_at,
        ))
        ->values()
        ->all(),
);

expect($response->json())
    ->ok->toBeTrue()
    ->channel->toBe('C08CL4QQH8F')
    ->message->ts->toBe('1739390364.067149')
    ->message->blocks->toBeArray()
    ->message->blocks->sequence(
        fn ($block) => $block->type === 'section' && $block->text->type === 'mrkdwn',
        fn ($block) => $block->type === 'divider',
        fn ($block) => $block->type === 'section' && $block->text->type === 'mrkdwn',
        fn ($block) => $block->type === 'actions' && $block->elements->sequence(
            fn ($element) => $element->type === 'button' && $element->text->type === 'plain_text',
            fn ($element) => $element->type === 'button' && $element->text->type === 'plain_text',
        ),
    );

$records = $this->connector
    ->send(new GetEmployeeReportRequest())
    ->collect('employees')
    ->filter(fn (array $employee) => ($this->filterEmployees)($employee))
    ->map(fn (array $employee) => [
        data_get($employee, 'employeeNumber'), // trigram
        $this->maxDate( // effective date
            value1: data_get($employee, 'customEffectiveDate'),
            value2: data_get($employee, 'employeeStatusDate'),
        ),
        null, // end date, leave blank
        min(max((float) data_get($employee, 'customWorkingTime'), 0), 100), // working time, has to be between 0 and 100
    ])
    ->toArray();
