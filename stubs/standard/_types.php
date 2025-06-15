<?php

namespace {
    /**
     * @template-covariant TKey
     * @template-covariant TValue
     * @template TSend
     * @template-covariant TReturn
     *
     * @template-implements Traversable<TKey, TValue>
     */
    class Generator implements Traversable
    {
        /**
         * @return ?TValue
         *
         * @psalm-ignore-nullable-return
         */
        public function current()
        {
        }

        /**
         * @return void
         */
        public function next()
        {
        }

        /**
         * @return TKey
         */
        public function key()
        {
        }

        /**
         * @return bool
         */
        public function valid()
        {
        }

        /**
         * @return void
         */
        public function rewind()
        {
        }

        /**
         * @return TReturn
         */
        public function getReturn()
        {
        }

        /**
         * @param TSend $value
         *
         * @return ?TValue
         *
         * @psalm-ignore-nullable-return
         */
        public function send($value)
        {
        }

        /**
         * @return ?TValue
         *
         * @psalm-ignore-nullable-return
         */
        public function throw(Throwable $exception)
        {
        }
    }
}
