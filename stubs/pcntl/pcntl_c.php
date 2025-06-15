<?php

namespace Pcntl;

enum QosClass
{
    case Background;
    case Utility;
    case Default;
    case UserInitiated;
    case UserInteractive;
}
